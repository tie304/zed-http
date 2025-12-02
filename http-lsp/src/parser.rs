use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};

extern "C" {
    fn tree_sitter_http() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_http() }
}

pub fn create_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(&language()).expect("Failed to set HTTP language");
    parser
}

pub fn parse(content: &str) -> Option<Tree> {
    let mut parser = create_parser();
    parser.parse(content, None)
}

/// Find all HTTP requests in the document
/// Returns: Vec<(start_line, end_line, method, url)>
pub fn find_requests(tree: &Tree, source: &str) -> Vec<HttpRequest> {
    let query_str = r#"
        (request
            (method) @method
            (target_url) @url
        ) @request
    "#;

    let query = match Query::new(&language(), query_str) {
        Ok(q) => q,
        Err(_) => return vec![],
    };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    let mut requests = Vec::new();
    let method_idx = query.capture_index_for_name("method").unwrap();
    let url_idx = query.capture_index_for_name("url").unwrap();
    let request_idx = query.capture_index_for_name("request").unwrap();

    while let Some(m) = matches.next() {
        let mut method = String::new();
        let mut url = String::new();
        let mut start_line = 0u32;
        let mut end_line = 0u32;

        for capture in m.captures {
            let text = capture
                .node
                .utf8_text(source.as_bytes())
                .unwrap_or("")
                .to_string();

            if capture.index == method_idx {
                method = text;
            } else if capture.index == url_idx {
                url = text;
            } else if capture.index == request_idx {
                start_line = capture.node.start_position().row as u32;
                end_line = capture.node.end_position().row as u32;
            }
        }

        if !method.is_empty() && !url.is_empty() {
            requests.push(HttpRequest {
                start_line,
                end_line,
                method,
                url,
            });
        }
    }

    requests
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub start_line: u32,
    pub end_line: u32,
    pub method: String,
    pub url: String,
}

/// Extract full request details from document at a specific line
pub fn extract_request_at_line(source: &str, line: u32) -> Option<FullHttpRequest> {
    let tree = parse(source)?;
    let requests = find_requests(&tree, source);

    for req in requests {
        if line >= req.start_line && line <= req.end_line {
            return extract_full_request(source, &req);
        }
    }

    None
}

fn extract_full_request(source: &str, req: &HttpRequest) -> Option<FullHttpRequest> {
    let lines: Vec<&str> = source.lines().collect();
    let start = req.start_line as usize;
    let end = (req.end_line as usize).min(lines.len());

    if start >= lines.len() {
        return None;
    }

    let mut headers = Vec::new();
    let mut body_lines = Vec::new();
    let mut in_body = false;

    for (i, line) in lines[start..=end].iter().enumerate() {
        if i == 0 {
            // Skip the request line (METHOD URL)
            continue;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            in_body = true;
            continue;
        }

        if in_body {
            body_lines.push(*line);
        } else if let Some((name, value)) = trimmed.split_once(':') {
            headers.push((name.trim().to_string(), value.trim().to_string()));
        }
    }

    Some(FullHttpRequest {
        method: req.method.clone(),
        url: req.url.clone(),
        headers,
        body: if body_lines.is_empty() {
            None
        } else {
            Some(body_lines.join("\n"))
        },
    })
}

#[derive(Debug, Clone)]
pub struct FullHttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}
