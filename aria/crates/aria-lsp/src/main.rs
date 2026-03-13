use std::sync::Arc;

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use aria_core::{checker::Diagnostic, checker::Severity, db::AriaDatabase};

mod diagnostics;
mod completions;
mod hover;

use diagnostics::validate_document;

/// Per-document cancellation token — value is incremented on each save to cancel
/// in-flight validation for the same document.
type CancelMap = Arc<DashMap<Url, u64>>;

pub struct AriaLsp {
    client: Client,
    db: Arc<std::sync::Mutex<AriaDatabase>>,
    cancel: CancelMap,
}

#[tower_lsp::async_trait]
impl LanguageServer for AriaLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "aria-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "aria-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.run_validation(uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.run_validation(uri, change.text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text.unwrap_or_default();
        self.run_validation(uri, text).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let items = completions::provide_completions(&uri, pos);
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        Ok(hover::provide_hover(&uri, pos))
    }
}

impl AriaLsp {
    async fn run_validation(&self, uri: Url, text: String) {
        // Increment cancel counter for this document
        let gen = {
            let mut entry = self.cancel.entry(uri.clone()).or_insert(0);
            *entry += 1;
            *entry
        };

        let cancel = self.cancel.clone();
        let client = self.client.clone();
        let db = self.db.clone();
        let uri_clone = uri.clone();

        tokio::spawn(async move {
            // Yield to allow new saves to increment the counter
            tokio::task::yield_now().await;

            // Check if cancelled (a newer save arrived)
            if cancel.get(&uri_clone).map(|v| *v) != Some(gen) {
                return;
            }

            let diagnostics = validate_document(&db, &uri_clone, &text);
            let lsp_diags: Vec<lsp_types::Diagnostic> = diagnostics
                .into_iter()
                .map(aria_diag_to_lsp)
                .collect();

            client
                .publish_diagnostics(uri_clone, lsp_diags, None)
                .await;
        });
    }
}

fn aria_diag_to_lsp(d: Diagnostic) -> lsp_types::Diagnostic {
    let severity = match d.severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warn => DiagnosticSeverity::WARNING,
    };

    let range = if let Some(r) = d.range {
        Range {
            start: Position {
                line: r.start_line,
                character: r.start_character,
            },
            end: Position {
                line: r.end_line,
                character: r.end_character,
            },
        }
    } else {
        let line = if d.line > 0 { d.line.saturating_sub(1) as u32 } else { 0 };
        Range {
            start: Position { line, character: 0 },
            end: Position { line, character: 0 },
        }
    };

    lsp_types::Diagnostic {
        range,
        severity: Some(severity),
        source: Some("aria-lsp".to_string()),
        message: d.message,
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let db = Arc::new(std::sync::Mutex::new(AriaDatabase::default()));
    let cancel: CancelMap = Arc::new(DashMap::new());

    let (service, socket) = LspService::new(|client| AriaLsp {
        client,
        db,
        cancel,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
