mod code_actions;
mod code_lens;
mod commands;
mod document;
mod hover;
mod http_client;
mod inlay_hints;
mod parser;

use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::code_actions::CodeActionsProvider;
use crate::code_lens::CodeLensProvider;
use crate::commands::CommandHandler;
use crate::document::DocumentManager;
use crate::hover::HoverProvider;
use crate::inlay_hints::InlayHintsProvider;

pub struct HttpLsp {
    client: Client,
    documents: Arc<DocumentManager>,
    code_lens: CodeLensProvider,
    code_actions: CodeActionsProvider,
    hover: HoverProvider,
    inlay_hints: InlayHintsProvider,
    commands: CommandHandler,
}

impl HttpLsp {
    pub fn new(client: Client) -> Self {
        let documents = Arc::new(DocumentManager::new());
        let code_lens = CodeLensProvider::new(Arc::clone(&documents));
        let code_actions = CodeActionsProvider::new(Arc::clone(&documents));
        let hover = HoverProvider::new(Arc::clone(&documents));
        let inlay_hints = InlayHintsProvider::new(Arc::clone(&documents));
        let commands = CommandHandler::new(Arc::clone(&documents), client.clone());

        Self {
            client,
            documents,
            code_lens,
            code_actions,
            hover,
            inlay_hints,
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
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![
                        "http.send".to_string(),
                        "http.show".to_string(),
                        "http.saveResponse".to_string(),
                        "http.showHeaders".to_string(),
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

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri.to_string();
        Ok(Some(self.inlay_hints.provide(&uri, params.range)))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.to_string();
        Ok(Some(self.code_actions.provide(&uri, params.range)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;
        Ok(self.hover.provide(&uri, position))
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
