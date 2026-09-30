//! The warm-up as seen by the client: which requests the server sends and
//! what it republishes once the index is built.

mod common;

use std::pin::Pin;
use std::time::Duration;

use common::TempDir;
use futures::{Sink, SinkExt, StreamExt};
use serde_json::{Value, json};
use stack_lang_server::server::Backend;
use tokio::sync::mpsc;
use tower::{Service, ServiceExt};
use tower_lsp::jsonrpc::{Id, Request, Response};
use tower_lsp::lsp_types::Url;
use tower_lsp::lsp_types::notification::{Notification, PublishDiagnostics};
use tower_lsp::{ExitedError, LspService};

const HELPER: &str = "func Helper(a, b) {\n}\n";
const MAIN: &str = "func Main() {\n  Helper(1);\n}\n";

struct Client {
    service: LspService<Backend>,
    from_server: mpsc::UnboundedReceiver<Request>,
    responses: Pin<Box<dyn Sink<Response, Error = ExitedError> + Send>>,
    next_id: i64,
}

impl Client {
    fn new() -> Self {
        let (service, socket) = LspService::new(Backend::new);
        let (mut requests, responses) = socket.split();
        // the server's channel holds one message: drain it while handlers run
        let (tx, from_server) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(request) = requests.next().await {
                if tx.send(request).is_err() {
                    break;
                }
            }
        });
        Client {
            service,
            from_server,
            responses: Box::pin(responses),
            next_id: 0,
        }
    }

    async fn call(&mut self, request: Request) -> Option<Response> {
        self.service
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap()
    }

    async fn request(&mut self, method: &'static str, params: Value) -> Response {
        self.next_id += 1;
        let request = Request::build(method)
            .params(params)
            .id(self.next_id)
            .finish();
        self.call(request).await.expect("a response")
    }

    async fn notify(&mut self, method: &'static str, params: Value) {
        self.call(Request::build(method).params(params).finish())
            .await;
    }

    async fn open(&mut self, uri: &Url, text: &str) {
        let params = json!({
            "textDocument": { "uri": uri, "languageId": "stack", "version": 1, "text": text }
        });
        self.notify("textDocument/didOpen", params).await;
    }

    async fn next_from_server(&mut self) -> Request {
        tokio::time::timeout(Duration::from_secs(30), self.from_server.recv())
            .await
            .expect("server went silent")
            .expect("server closed")
    }

    async fn reply(&mut self, id: Id, result: Value) {
        self.responses
            .send(Response::from_ok(id, result))
            .await
            .unwrap();
    }
}

/// Everything the server sent until the warm-up cleared the status bar
/// and made the expected number of requests.
#[derive(Default)]
struct WarmUp {
    /// Methods of the requests, in order.
    requests: Vec<String>,
    /// Diagnostic messages per publish for the watched uri, in order,
    /// each tagged with how many requests came before it.
    published: Vec<(usize, Vec<String>)>,
}

impl WarmUp {
    async fn record(
        client: &mut Client,
        uri: &Url,
        expected_requests: usize,
        mut answer: impl FnMut(&str) -> Value,
    ) -> Self {
        let mut warm_up = WarmUp::default();
        let mut cleared = false;
        while !cleared || warm_up.requests.len() < expected_requests {
            let message = client.next_from_server().await;
            let params = message.params().cloned().unwrap_or_default();
            match (message.method(), message.id().cloned()) {
                (method, Some(id)) => {
                    warm_up.requests.push(method.to_owned());
                    let result = answer(method);
                    client.reply(id, result).await;
                }
                (PublishDiagnostics::METHOD, None) if params["uri"] == json!(uri) => {
                    let messages = params["diagnostics"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|d| d["message"].as_str().unwrap().to_owned())
                        .collect();
                    warm_up.published.push((warm_up.requests.len(), messages));
                }
                ("custom/statusBar", None) if params["text"] == "" => cleared = true,
                _ => {}
            }
        }
        warm_up
    }
}

fn is_arity_error(message: &str) -> bool {
    message.contains("Helper") && message.contains("1 argument")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn warm_up_takes_settings_from_initialize_without_asking_the_client() {
    let dir = TempDir::new("server_init_options");
    dir.write("lib.prg", HELPER);
    let folder = Url::from_file_path(&dir.0).unwrap();
    let main = Url::from_file_path(dir.0.join("main.prg")).unwrap();

    let mut client = Client::new();
    let initialize = json!({
        "capabilities": { "workspace": {
            "codeLens": { "refreshSupport": true },
            "semanticTokens": { "refreshSupport": true },
        } },
        "workspaceFolders": [{ "uri": folder, "name": "w" }],
        "initializationOptions": { "lens_enabled": true, "ini_path": "" },
    });
    client.request("initialize", initialize).await;
    // opened before the warm-up starts, so linted against an empty index
    client.open(&main, MAIN).await;
    client.notify("initialized", json!({})).await;

    let mut warm_up = WarmUp::record(&mut client, &main, 2, |method| match method {
        "workspace/codeLens/refresh" | "workspace/semanticTokens/refresh" => Value::Null,
        other => panic!("unexpected request to the client: {other}"),
    })
    .await;

    warm_up.requests.sort();
    assert_eq!(
        warm_up.requests,
        [
            "workspace/codeLens/refresh",
            "workspace/semanticTokens/refresh"
        ]
    );
    let [(_, cold), (before_refresh, warm)] = warm_up.published.as_slice() else {
        panic!("expected a publish on open and after warm-up");
    };
    assert!(!cold.iter().any(|m| is_arity_error(m)), "got {cold:?}");
    assert!(warm.iter().any(|m| is_arity_error(m)), "got {warm:?}");
    assert_eq!(
        *before_refresh, 0,
        "diagnostics go out before the refreshes"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn warm_up_asks_the_client_when_initialize_has_no_settings() {
    // an older extension: no `ini_path`, no refresh support
    let dir = TempDir::new("server_fallback");
    dir.write("lib.prg", HELPER);
    let folder = Url::from_file_path(&dir.0).unwrap();
    let main = Url::from_file_path(dir.0.join("main.prg")).unwrap();

    let mut client = Client::new();
    let initialize = json!({
        "capabilities": {},
        "initializationOptions": { "lens_enabled": true },
    });
    client.request("initialize", initialize).await;
    client.open(&main, MAIN).await;
    client.notify("initialized", json!({})).await;

    let warm_up = WarmUp::record(&mut client, &main, 2, |method| match method {
        "workspace/configuration" => json!([""]),
        "workspace/workspaceFolders" => json!([{ "uri": folder, "name": "w" }]),
        other => panic!("unexpected request to the client: {other}"),
    })
    .await;

    assert_eq!(
        warm_up.requests,
        ["workspace/configuration", "workspace/workspaceFolders"]
    );
    let (_, warm) = warm_up.published.last().expect("a publish after warm-up");
    assert!(warm.iter().any(|m| is_arity_error(m)), "got {warm:?}");
}
