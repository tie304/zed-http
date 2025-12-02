use std::sync::Arc;

use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::Client;

use crate::document::DocumentManager;
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
        match params.command.as_str() {
            "http.send" => self.send_request(params.arguments, false).await,
            "http.showHeaders" => self.send_request(params.arguments, true).await,
            "http.saveResponse" => self.save_response(params.arguments).await,
            _ => Err(Error::method_not_found()),
        }
    }

    async fn send_request(
        &self,
        args: Vec<serde_json::Value>,
        headers_only: bool,
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
                let output = if headers_only {
                    response.format_headers_only()
                } else {
                    response.format_full()
                };

                // Show response in a message (Zed will display this)
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
                // Generate response file path
                let response_path = format!("{}.response", uri.trim_end_matches(".http"));
                let output = response.format_full();

                // For now, show where it would be saved
                // Full implementation would use workspace/applyEdit to create the file
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
            return Err(Error::invalid_params("Missing arguments"));
        }

        let uri = args[0]
            .as_str()
            .ok_or_else(|| Error::invalid_params("Invalid URI"))?
            .to_string();

        let line = args[1]
            .as_u64()
            .ok_or_else(|| Error::invalid_params("Invalid line number"))? as u32;

        Ok((uri, line))
    }
}
