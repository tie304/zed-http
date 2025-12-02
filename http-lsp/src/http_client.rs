use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method};

use crate::parser::FullHttpRequest;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u128,
}

impl HttpResponse {
    pub fn format_full(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "HTTP {} {}\n",
            self.status, self.status_text
        ));
        output.push_str(&format!("Duration: {}ms\n\n", self.duration_ms));

        output.push_str("--- Headers ---\n");
        for (name, value) in &self.headers {
            output.push_str(&format!("{}: {}\n", name, value));
        }

        output.push_str("\n--- Body ---\n");
        output.push_str(&self.body);

        output
    }

    pub fn format_headers_only(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "HTTP {} {}\n",
            self.status, self.status_text
        ));
        output.push_str(&format!("Duration: {}ms\n\n", self.duration_ms));

        for (name, value) in &self.headers {
            output.push_str(&format!("{}: {}\n", name, value));
        }

        output
    }
}

pub async fn execute_request(request: &FullHttpRequest) -> Result<HttpResponse, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let method = match request.method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        other => return Err(format!("Unsupported HTTP method: {}", other)),
    };

    let mut headers = HeaderMap::new();
    for (name, value) in &request.headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| format!("Invalid header name '{}': {}", name, e))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|e| format!("Invalid header value for '{}': {}", name, e))?;
        headers.insert(header_name, header_value);
    }

    let mut req_builder = client.request(method, &request.url).headers(headers);

    if let Some(body) = &request.body {
        req_builder = req_builder.body(body.clone());
    }

    let start = Instant::now();
    let response = req_builder.send().await.map_err(|e| e.to_string())?;
    let duration_ms = start.elapsed().as_millis();

    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("Unknown")
        .to_string();

    let response_headers: Vec<(String, String)> = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(HttpResponse {
        status,
        status_text,
        headers: response_headers,
        body,
        duration_ms,
    })
}
