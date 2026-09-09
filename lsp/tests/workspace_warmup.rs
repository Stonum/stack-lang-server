//! Covers the "serve while the cache is still warming up" contract that
//! `initialized` relies on: the server discovers workspace files eagerly but
//! builds their semantic models lazily/among other work, so every query must
//! return promptly against whatever the cache holds so far -- never block
//! waiting for a full warm-up.

mod common;

use std::fs;

use common::{temp_uri, text_document};
use stack_lang_server::workspace::Workspace;
use tower_lsp::lsp_types::{DocumentSymbolResponse, Url, WorkspaceFolder};

struct TempDir(std::path::PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("sls_warmup_{}_{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp workspace dir");
        TempDir(dir)
    }

    fn write(&self, name: &str, contents: &str) {
        fs::write(self.0.join(name), contents).expect("write temp workspace file");
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn queries_serve_from_a_partially_warmed_cache() {
    let dir = TempDir::new("partial");
    dir.write("a.prg", "func Foo() {\n}\n");
    dir.write("b.prg", "func Bar() {\n}\n");

    let workspace = Workspace::new();
    let folder = WorkspaceFolder {
        uri: Url::from_file_path(&dir.0).unwrap(),
        name: "w".to_string(),
    };
    workspace
        .init_with_workspace_folders(Some(vec![folder]))
        .await
        .expect("folder init should succeed");

    // Files are discovered, but no semantic model is built yet: a workspace
    // symbol query must come back immediately and empty, not stall.
    let before = workspace
        .symbol_information("")
        .await
        .expect("symbol_information should return Some");
    assert!(before.is_empty(), "nothing warmed yet, got {before:?}");

    // A document opened right now still yields its own symbols, parsed
    // straight from the buffer, with the workspace cache still cold.
    let uri = temp_uri("warmup_open.prg");
    workspace
        .open_document(text_document(uri.clone(), "mlang", "func Baz() {\n}\n"))
        .await
        .unwrap();
    let doc_symbols = workspace
        .document_symbol_response(&uri)
        .await
        .expect("document_symbol should not error")
        .expect("an open mlang document has a symbol response");
    let names: Vec<String> = match doc_symbols {
        DocumentSymbolResponse::Flat(flat) => flat.into_iter().map(|s| s.name).collect(),
        DocumentSymbolResponse::Nested(nested) => nested.into_iter().map(|s| s.name).collect(),
    };
    assert!(names.iter().any(|n| n == "Baz"), "got {names:?}");

    // Warm-up completes -> the same query now sees the workspace files.
    workspace.update_semantic_information().await;
    let after = workspace
        .symbol_information("")
        .await
        .expect("symbol_information should return Some");
    let after_names: Vec<&str> = after.iter().map(|s| s.name.as_str()).collect();
    assert!(after_names.contains(&"Foo"), "got {after_names:?}");
    assert!(after_names.contains(&"Bar"), "got {after_names:?}");
}
