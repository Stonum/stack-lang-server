use chrono::Local;
use std::env;
use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

use stack_lang_server::server::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    init_logger();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(Backend::new).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

fn init_logger() {
    let mut builder = Builder::from_default_env();

    // Set default level to info if not specified
    if env::var("RUST_LOG").is_err() {
        builder.filter_level(LevelFilter::Info);
    }

    builder.format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.args()
        )
    });

    builder.init();
}
