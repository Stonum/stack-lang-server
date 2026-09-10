//! Covers the "serve while the cache is still warming up" contract that
//! `initialized` relies on: the server discovers workspace files eagerly but
//! builds their semantic models lazily/among other work, so every query must
//! return promptly against whatever the cache holds so far -- never block
//! waiting for a full warm-up.

mod common;

use std::fs;
use std::sync::Arc;
use std::time::Duration;

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
async fn init_from_settings_file_discovers_prg_files() {
    let dir = TempDir::new("ini");
    dir.write("one.prg", "func One() {\n}\n");
    dir.write("two.prg", "func Two() {\n}\n");
    dir.write(
        "stack.ini",
        &format!("[AppPath]\nPRG={}\n", dir.0.join("**").display()),
    );

    let workspace = Workspace::new();
    workspace
        .init_with_settings_file(&dir.0.to_string_lossy())
        .await
        .expect("ini init should succeed");

    workspace.update_semantic_information().await;

    let names: Vec<String> = workspace
        .symbol_information("")
        .await
        .expect("Some")
        .into_iter()
        .map(|s| s.name)
        .collect();
    assert!(names.iter().any(|n| n == "One"), "got {names:?}");
    assert!(names.iter().any(|n| n == "Two"), "got {names:?}");
}

#[tokio::test]
async fn init_from_a_missing_settings_file_errors_without_panic() {
    let workspace = Workspace::new();
    let missing = std::env::temp_dir().join("sls_warmup_definitely_missing_dir");

    let result = workspace
        .init_with_settings_file(&missing.to_string_lossy())
        .await;

    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn warm_up_runs_concurrently_with_document_changes() {
    // Concurrency smoke test for the warm-up path: `update_semantic_information`
    // and a storm of `change_document` writes run at the same time, both must
    // finish (timeout guards a hang) and leave the cache consistent and
    // queryable. Exercises the shard-snapshot in `update_semantic_information`
    // that keeps it from parking on a `DashMap` guard across `.await`.
    let dir = TempDir::new("concurrent");
    for i in 0..300 {
        dir.write(&format!("f{i}.prg"), &format!("func F{i}() {{\n}}\n"));
    }

    let workspace = Arc::new(Workspace::new());
    let folder = WorkspaceFolder {
        uri: Url::from_file_path(&dir.0).unwrap(),
        name: "w".to_string(),
    };
    workspace
        .init_with_workspace_folders(Some(vec![folder]))
        .await
        .unwrap();

    let warming = {
        let workspace = Arc::clone(&workspace);
        tokio::spawn(async move { workspace.update_semantic_information().await })
    };

    let churning = {
        let workspace = Arc::clone(&workspace);
        let root = dir.0.clone();
        tokio::spawn(async move {
            // Hammer writes across every file (hence every `DashMap` shard),
            // so whichever shard the warm-up is parked on gets hit.
            for round in 0..3 {
                for i in 0..300 {
                    let uri = Url::from_file_path(root.join(format!("f{i}.prg"))).unwrap();
                    let text = format!("func C{round}_{i}() {{\n}}\n");
                    workspace
                        .change_document(text_document(uri, "mlang", &text))
                        .await
                        .unwrap();
                }
            }
        })
    };

    tokio::time::timeout(Duration::from_secs(30), async {
        warming.await.unwrap();
        churning.await.unwrap();
    })
    .await
    .expect("warm-up must not deadlock against concurrent edits");

    // Cache is still coherent afterwards: every one of the 300 files resolves
    // to at least its function symbol, whichever writer landed last.
    let symbols = workspace.symbol_information("").await.expect("Some");
    assert!(
        symbols.len() >= 300,
        "expected a symbol per file, got {}",
        symbols.len()
    );
}

#[tokio::test]
async fn open_document_lints_calls_against_workspace_definitions() {
    // `open_document` computes diagnostics on a blocking worker, linting the
    // buffer against the warmed cross-file semantic cache. Prove that path
    // still wires the workspace definitions in: an arity error on a call to a
    // function defined in *another* workspace file.
    let dir = TempDir::new("xfile_lint");
    dir.write("lib.prg", "func Helper(a, b) {\n}\n");

    let workspace = Workspace::new();
    let folder = WorkspaceFolder {
        uri: Url::from_file_path(&dir.0).unwrap(),
        name: "w".to_string(),
    };
    workspace
        .init_with_workspace_folders(Some(vec![folder]))
        .await
        .unwrap();
    workspace.update_semantic_information().await;

    let main_uri = Url::from_file_path(dir.0.join("main.prg")).unwrap();
    let diagnostics = workspace
        .open_document(text_document(
            main_uri,
            "mlang",
            "func Main() {\n  Helper(1);\n}\n",
        ))
        .await
        .expect("open_document should not error");

    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Helper") && d.message.contains("1 argument")),
        "expected a cross-file arity diagnostic, got {diagnostics:?}"
    );
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
