use std::sync::Arc;
use tower_lsp::lsp_types::*;

use crate::document::DocumentManager;

pub struct CodeActionsProvider {
    documents: Arc<DocumentManager>,
}

impl CodeActionsProvider {
    pub fn new(documents: Arc<DocumentManager>) -> Self {
        Self { documents }
    }

    pub fn provide(&self, uri: &str, range: Range) -> Vec<CodeActionOrCommand> {
        let requests = self.documents.get_requests(uri);
        let mut actions = Vec::new();

        // Find request at cursor position (match anywhere within request block)
        for req in requests {
            let cursor_line = range.start.line;
            if cursor_line >= req.start_line && cursor_line <= req.end_line {
                // Send Request
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "▶ Send Request".to_string(),
                    kind: Some(CodeActionKind::EMPTY),
                    command: Some(Command {
                        title: "Send".to_string(),
                        command: "http.send".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                }));

                // Show Response
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "👁 Show Response".to_string(),
                    kind: Some(CodeActionKind::EMPTY),
                    command: Some(Command {
                        title: "Show".to_string(),
                        command: "http.show".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                }));

                // Save Response
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "💾 Save Response".to_string(),
                    kind: Some(CodeActionKind::EMPTY),
                    command: Some(Command {
                        title: "Save".to_string(),
                        command: "http.saveResponse".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                }));

                // Show Headers
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "◉ Show Headers".to_string(),
                    kind: Some(CodeActionKind::EMPTY),
                    command: Some(Command {
                        title: "Headers".to_string(),
                        command: "http.showHeaders".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri),
                            serde_json::json!(req.start_line),
                        ]),
                    }),
                    ..Default::default()
                }));

                break;
            }
        }

        actions
    }
}
