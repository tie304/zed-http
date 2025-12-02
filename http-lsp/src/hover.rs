use std::sync::Arc;
use tower_lsp::lsp_types::*;

use crate::document::DocumentManager;

pub struct HoverProvider {
    documents: Arc<DocumentManager>,
}

impl HoverProvider {
    pub fn new(documents: Arc<DocumentManager>) -> Self {
        Self { documents }
    }

    pub fn provide(&self, uri: &str, position: Position) -> Option<Hover> {
        let requests = self.documents.get_requests(uri);

        for req in requests {
            // Check if hovering on request line
            if position.line == req.start_line {
                let cached = self.documents.get_cached_response(uri, req.start_line);

                let status_info = if let Some(resp) = &cached {
                    format!("**Last Response:** {} {} ({}ms)\n\n---\n\n",
                        resp.status, resp.status_text, resp.duration_ms)
                } else {
                    String::new()
                };

                // Encode args for command URIs
                let args = format!("[\"{}\",{}]", uri, req.start_line);
                let encoded_args = urlencoding::encode(&args);

                let content = format!(
                    "## {} {}\n\n{}\
                    [▶ Send](command:http.send?{}) | \
                    [👁 Show](command:http.show?{}) | \
                    [💾 Save](command:http.saveResponse?{}) | \
                    [◉ Headers](command:http.showHeaders?{})",
                    req.method,
                    req.url,
                    status_info,
                    encoded_args,
                    encoded_args,
                    encoded_args,
                    encoded_args
                );

                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: content,
                    }),
                    range: Some(Range {
                        start: Position { line: req.start_line, character: 0 },
                        end: Position { line: req.start_line, character: (req.method.len() + 1 + req.url.len()) as u32 },
                    }),
                });
            }
        }

        None
    }
}
