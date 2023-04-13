mod cache;
use cache::SymbolCache;

mod skill;
use skill::{parse_global_symbols, parse_skill};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower_lsp::jsonrpc::{Error, ErrorCode, Result};
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use log::{debug, info};
use walkdir::WalkDir;

extern crate glob;
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Debug)]
struct Backend {
    client: Client,
    cache: SymbolCache,
}

#[derive(Debug, Deserialize, Serialize)]
struct CustomNotificationParams {
    title: String,
    message: String,
}

impl CustomNotificationParams {
    fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        CustomNotificationParams {
            title: title.into(),
            message: message.into(),
        }
    }
}

enum CustomNotification {}

impl Notification for CustomNotification {
    type Params = CustomNotificationParams;

    const METHOD: &'static str = "custom/notification";
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, init_params: InitializeParams) -> Result<InitializeResult> {
        let root = init_params
            .root_uri
            .ok_or(Error::new(ErrorCode::InvalidParams))?;
        info!(target: "Backend", "Initializing Language Server");

        let root_dir = root.path().to_string();

        info!(target: "Backend", "Caching started in '{}'", root_dir);

        for entry in WalkDir::new(root_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_path = entry.path().to_str();
            match f_path {
                Some(path) => {
                    if path.ends_with(".il") {
                        info!("found '{}'", path);
                        self.cache.update(path);
                    }
                }
                None => {}
            }
        }
        info!(target: "Backend", "Caching finished. Found {} files.", self.cache.symbols.len());

        debug!(target: "Backend", "{:?}", self.cache.symbols);

        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["custom/notification".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["(".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
        self.client
            .send_notification::<CustomNotification>(CustomNotificationParams::new(
                "title", "message",
            ))
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        if params.command == "custom.notification" {
            self.client
                .send_notification::<CustomNotification>(CustomNotificationParams::new(
                    "Hello", "Message",
                ))
                .await;
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("Command executed with params: {params:?}"),
                )
                .await;
            Ok(None)
        } else {
            Err(Error::invalid_request())
        }
    }

    async fn completion(&self, cparams: CompletionParams) -> Result<Option<CompletionResponse>> {
        let symbols: Vec<CompletionItem> = self
            .cache
            .symbols
            .get_mut(&cparams.text_document_position.text_document.uri.to_string())
            .unwrap()
            .to_vec();

        Ok(Some(CompletionResponse::Array(symbols)))
    }
}

#[tokio::main]
async fn main() {
    let writer = tracing_appender::rolling::never(".", "srls.out");
    tracing_subscriber::fmt().with_writer(writer).init();
    info!(target: "main", "Starting");

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        cache: SymbolCache::new(),
    });
    info!("Creating server instance.");
    Server::new(stdin, stdout, socket).serve(service).await;
}
