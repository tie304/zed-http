use std::sync::Arc;
use tower_lsp::lsp_types::*;

use crate::document::DocumentManager;

pub struct CodeLensProvider {
    documents: Arc<DocumentManager>,
}

impl CodeLensProvider {
    pub fn new(documents: Arc<DocumentManager>) -> Self {
        Self { documents }
    }

    pub fn provide(&self, uri: &str) -> Vec<CodeLens> {
        let requests = self.documents.get_requests(uri);
        let mut lenses = Vec::new();

        for req in requests {
            let range = Range {
                start: Position {
                    line: req.start_line,
                    character: 0,
                },
                end: Position {
                    line: req.start_line,
                    character: req.method.len() as u32,
                },
            };

            // Send Request button
            lenses.push(CodeLens {
                range,
                command: Some(Command {
                    title: "▶ Send".to_string(),
                    command: "http.send".to_string(),
                    arguments: Some(vec![
                        serde_json::json!(uri),
                        serde_json::json!(req.start_line),
                    ]),
                }),
                data: None,
            });

            // Show Response button (cached)
            lenses.push(CodeLens {
                range,
                command: Some(Command {
                    title: "👁 Show".to_string(),
                    command: "http.show".to_string(),
                    arguments: Some(vec![
                        serde_json::json!(uri),
                        serde_json::json!(req.start_line),
                    ]),
                }),
                data: None,
            });

            // Save Response button
            lenses.push(CodeLens {
                range,
                command: Some(Command {
                    title: "💾 Save".to_string(),
                    command: "http.saveResponse".to_string(),
                    arguments: Some(vec![
                        serde_json::json!(uri),
                        serde_json::json!(req.start_line),
                    ]),
                }),
                data: None,
            });

            // Show Headers button
            lenses.push(CodeLens {
                range,
                command: Some(Command {
                    title: "◉ Headers".to_string(),
                    command: "http.showHeaders".to_string(),
                    arguments: Some(vec![
                        serde_json::json!(uri),
                        serde_json::json!(req.start_line),
                    ]),
                }),
                data: None,
            });
        }

        lenses
    }
}
