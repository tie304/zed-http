use std::sync::Arc;

use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::Client;

use crate::document::{CachedResponse, DocumentManager};
use crate::http_client;
use crate::parser;

pub struct CommandHandler {
    documents: Arc<DocumentManager>,
    client: Client,
}

impl CommandHandler {
    pub fn new(documents: Arc<DocumentManager>, client: Client) -> Self {
        Self { documents, client }
    }

    pub async fn execute(&self, params: ExecuteCommandParams) -> Result<Option<serde_json::Value>> {
        self.client
            .log_message(
                MessageType::INFO,
                format!("Executing command: {} with args: {:?}", params.command, params.arguments),
            )
            .await;

        let result = match params.command.as_str() {
            "http.send" => self.send_request(params.arguments).await,
            "http.show" => self.show_response(params.arguments, false).await,
            "http.saveResponse" => self.save_response(params.arguments).await,
            "http.showHeaders" => self.show_response(params.arguments, true).await,
            _ => Err(Error::method_not_found()),
        };

        if let Err(ref e) = result {
            self.client
                .log_message(MessageType::ERROR, format!("Command error: {:?}", e))
                .await;
        }

        result
    }

    async fn send_request(
        &self,
        args: Vec<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>> {
        let (uri, line) = self.parse_args(&args)?;

        let content = self
            .documents
            .get_content(&uri)
            .ok_or_else(|| Error::invalid_params("Document not found"))?;

        let request = parser::extract_request_at_line(&content, line)
            .ok_or_else(|| Error::invalid_params("No request found at line"))?;

        self.client
            .log_message(
                MessageType::INFO,
                format!("Executing {} {}", request.method, request.url),
            )
            .await;

        match http_client::execute_request(&request).await {
            Ok(response) => {
                // Cache the response
                self.documents.cache_response(
                    &uri,
                    line,
                    CachedResponse {
                        status: response.status,
                        status_text: response.status_text.clone(),
                        headers: response.headers.clone(),
                        body: response.body.clone(),
                        duration_ms: response.duration_ms,
                    },
                );

                let output = response.format_full();

                self.client
                    .show_message(MessageType::INFO, &output)
                    .await;

                Ok(Some(serde_json::json!({
                    "status": response.status,
                    "body": response.body,
                    "duration_ms": response.duration_ms
                })))
            }
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Request failed: {}", e))
                    .await;
                Err(Error::internal_error())
            }
        }
    }

    async fn show_response(
        &self,
        args: Vec<serde_json::Value>,
        headers_only: bool,
    ) -> Result<Option<serde_json::Value>> {
        let (uri, line) = self.parse_args(&args)?;

        match self.documents.get_cached_response(&uri, line) {
            Some(cached) => {
                let output = if headers_only {
                    format_headers_only(&cached)
                } else {
                    format_full(&cached)
                };

                self.client
                    .show_message(MessageType::INFO, &output)
                    .await;

                Ok(Some(serde_json::json!({
                    "status": cached.status,
                    "body": cached.body,
                    "duration_ms": cached.duration_ms
                })))
            }
            None => {
                self.client
                    .show_message(MessageType::WARNING, "No cached response. Send the request first.")
                    .await;
                Ok(None)
            }
        }
    }

    async fn save_response(&self, args: Vec<serde_json::Value>) -> Result<Option<serde_json::Value>> {
        let (uri, line) = self.parse_args(&args)?;

        let content = self
            .documents
            .get_content(&uri)
            .ok_or_else(|| Error::invalid_params("Document not found"))?;

        let request = parser::extract_request_at_line(&content, line)
            .ok_or_else(|| Error::invalid_params("No request found at line"))?;

        self.client
            .log_message(
                MessageType::INFO,
                format!("Executing {} {} (saving response)", request.method, request.url),
            )
            .await;

        match http_client::execute_request(&request).await {
            Ok(response) => {
                // Cache the response
                self.documents.cache_response(
                    &uri,
                    line,
                    CachedResponse {
                        status: response.status,
                        status_text: response.status_text.clone(),
                        headers: response.headers.clone(),
                        body: response.body.clone(),
                        duration_ms: response.duration_ms,
                    },
                );

                // Generate response file path
                let response_path = format!("{}.response", uri.trim_end_matches(".http"));
                let output = response.format_full();

                self.client
                    .show_message(
                        MessageType::INFO,
                        format!("Response saved to: {}\n\n{}", response_path, output),
                    )
                    .await;

                Ok(Some(serde_json::json!({
                    "saved_to": response_path,
                    "status": response.status
                })))
            }
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Request failed: {}", e))
                    .await;
                Err(Error::internal_error())
            }
        }
    }

    fn parse_args(&self, args: &[serde_json::Value]) -> Result<(String, u32)> {
        if args.len() < 2 {
            return Err(Error::invalid_params(format!(
                "Missing arguments: expected 2, got {}. Args: {:?}",
                args.len(),
                args
            )));
        }

        let uri = args[0]
            .as_str()
            .ok_or_else(|| Error::invalid_params(format!("Invalid URI: {:?}", args[0])))?
            .to_string();

        let line = args[1]
            .as_u64()
            .ok_or_else(|| Error::invalid_params(format!("Invalid line number: {:?}", args[1])))?
            as u32;

        Ok((uri, line))
    }
}

fn format_full(response: &CachedResponse) -> String {
    let mut output = String::new();
    output.push_str(&format!("HTTP {} {}\n", response.status, response.status_text));
    output.push_str(&format!("Duration: {}ms\n\n", response.duration_ms));
    output.push_str("--- Headers ---\n");
    for (name, value) in &response.headers {
        output.push_str(&format!("{}: {}\n", name, value));
    }
    output.push_str("\n--- Body ---\n");
    output.push_str(&response.body);
    output
}

fn format_headers_only(response: &CachedResponse) -> String {
    let mut output = String::new();
    output.push_str(&format!("HTTP {} {}\n", response.status, response.status_text));
    output.push_str(&format!("Duration: {}ms\n\n", response.duration_ms));
    for (name, value) in &response.headers {
        output.push_str(&format!("{}: {}\n", name, value));
    }
    output
}
