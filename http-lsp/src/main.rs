mod code_lens;
mod commands;
mod document;
mod http_client;
mod parser;

use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::code_lens::CodeLensProvider;
use crate::commands::CommandHandler;
use crate::document::DocumentManager;

pub struct HttpLsp {
    client: Client,
    documents: Arc<DocumentManager>,
    code_lens: CodeLensProvider,
    commands: CommandHandler,
}

impl HttpLsp {
    pub fn new(client: Client) -> Self {
        let documents = Arc::new(DocumentManager::new());
        let code_lens = CodeLensProvider::new(Arc::clone(&documents));
        let commands = CommandHandler::new(Arc::clone(&documents), client.clone());

        Self {
            client,
            documents,
            code_lens,
            commands,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for HttpLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![
                        "http.send".to_string(),
                        "http.showHeaders".to_string(),
                        "http.saveResponse".to_string(),
                    ],
                    work_done_progress_options: Default::default(),
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "http-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "HTTP LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.text_document.text;
        self.documents.open(uri, content);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(change) = params.content_changes.into_iter().next() {
            self.documents.update(&uri, change.text);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.close(&uri);
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri.to_string();
        Ok(Some(self.code_lens.provide(&uri)))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<serde_json::Value>> {
        self.commands.execute(params).await
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(HttpLsp::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
