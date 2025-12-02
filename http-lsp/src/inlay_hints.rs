use std::sync::Arc;
use tower_lsp::lsp_types::*;

use crate::document::DocumentManager;

pub struct InlayHintsProvider {
    documents: Arc<DocumentManager>,
}

impl InlayHintsProvider {
    pub fn new(documents: Arc<DocumentManager>) -> Self {
        Self { documents }
    }

    pub fn provide(&self, uri: &str, _range: Range) -> Vec<InlayHint> {
        let requests = self.documents.get_requests(uri);
        let mut hints = Vec::new();

        for req in requests {
            // Position at end of the request line (after URL)
            let position = Position {
                line: req.start_line,
                character: (req.method.len() + 1 + req.url.len()) as u32,
            };

            // Create clickable label parts for each button
            let label_parts = vec![
                InlayHintLabelPart {
                    value: "  ".to_string(),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: "▶ Send".to_string(),
                    tooltip: Some(InlayHintLabelPartTooltip::String(
                        "Execute HTTP request".to_string(),
                    )),
                    command: Some(Command {
                        title: "Send".to_string(),
                        command: "http.send".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: " | ".to_string(),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: "👁 Show".to_string(),
                    tooltip: Some(InlayHintLabelPartTooltip::String(
                        "Show cached response".to_string(),
                    )),
                    command: Some(Command {
                        title: "Show".to_string(),
                        command: "http.show".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: " | ".to_string(),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: "💾 Save".to_string(),
                    tooltip: Some(InlayHintLabelPartTooltip::String(
                        "Save response to file".to_string(),
                    )),
                    command: Some(Command {
                        title: "Save".to_string(),
                        command: "http.saveResponse".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: " | ".to_string(),
                    ..Default::default()
                },
                InlayHintLabelPart {
                    value: "◉ Headers".to_string(),
                    tooltip: Some(InlayHintLabelPartTooltip::String(
                        "Show response headers only".to_string(),
                    )),
                    command: Some(Command {
                        title: "Headers".to_string(),
                        command: "http.showHeaders".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                },
            ];

            hints.push(InlayHint {
                position,
                label: InlayHintLabel::LabelParts(label_parts),
                kind: None,
                text_edits: None,
                tooltip: None,
                padding_left: Some(true),
                padding_right: None,
                data: None,
            });
        }

        hints
    }
}
