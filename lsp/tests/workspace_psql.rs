//! End-to-end tests for `Workspace`'s public API against psql (`.sql`)
//! documents -- everything here works purely off the text carried in the
//! LSP request payload (`TextDocumentItem::text`), the same way a real
//! editor sends it; nothing is read from disk. `Url::to_file_path()` is
//! only used internally to pick the file extension, so the path doesn't
//! need to exist.
//!
//! The main thing under test: mlang-only features (hover, completion,
//! signature help, semantic tokens) must degrade gracefully (`None`/empty)
//! for a psql document instead of panicking -- this is exactly the class
//! of bug `CurrentDocument::syntax()` used to have before it became
//! `mlang_syntax() -> Option<_>`.

mod common;

use common::{formatting_options, temp_uri, text_document, whole_document_range};
use stack_lang_server::workspace::Workspace;
use tower_lsp::lsp_types::{Position, Range};

#[tokio::test]
async fn opens_a_sql_document_via_workspace() {
    let workspace = Workspace::new();
    let uri = temp_uri("opens_a_sql_document_via_workspace.sql");

    let diagnostics = workspace
        .open_document(text_document(uri, "sql", "select a from t;\n"))
        .await
        .expect("open_document should not error");

    assert!(diagnostics.is_empty());
}

#[tokio::test]
async fn opening_malformed_sql_reports_diagnostics_not_a_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("opening_malformed_sql_reports_diagnostics_not_a_panic.sql");

    let diagnostics = workspace
        .open_document(text_document(uri, "sql", "select from where;;;\n"))
        .await
        .expect("open_document should not error, even for invalid SQL");

    assert!(!diagnostics.is_empty());
    assert_eq!(diagnostics[0].source.as_deref(), Some("psql-parser"));
}

#[tokio::test]
async fn hover_on_a_sql_document_returns_none_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("hover_on_a_sql_document_returns_none_not_panic.sql");

    workspace
        .open_document(text_document(uri.clone(), "sql", "select a from t;\n"))
        .await
        .unwrap();

    let hover = workspace
        .hover(&uri, Position::new(0, 7))
        .await
        .expect("hover should not error on a psql document");

    assert!(hover.is_none());
}

#[tokio::test]
async fn completion_on_a_sql_document_returns_none_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("completion_on_a_sql_document_returns_none_not_panic.sql");

    workspace
        .open_document(text_document(uri.clone(), "sql", "select a from t;\n"))
        .await
        .unwrap();

    let completion = workspace
        .completion(&uri, Position::new(0, 7))
        .await
        .expect("completion should not error on a psql document");

    assert!(completion.is_none());
}

#[tokio::test]
async fn signature_help_on_a_sql_document_returns_none_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("signature_help_on_a_sql_document_returns_none_not_panic.sql");

    workspace
        .open_document(text_document(uri.clone(), "sql", "select a from t;\n"))
        .await
        .unwrap();

    let signature_help = workspace
        .signature_help(&uri, Position::new(0, 7))
        .await
        .expect("signature_help should not error on a psql document");

    assert!(signature_help.is_none());
}

#[tokio::test]
async fn semantic_tokens_on_a_sql_document_returns_none_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("semantic_tokens_on_a_sql_document_returns_none_not_panic.sql");

    workspace
        .open_document(text_document(uri.clone(), "sql", "select a from t;\n"))
        .await
        .unwrap();

    let tokens = workspace
        .semantic_tokens(&uri, None)
        .await
        .expect("semantic_tokens should not error on a psql document");

    assert!(tokens.is_none());
}

#[tokio::test]
async fn format_a_sql_document_via_workspace() {
    let workspace = Workspace::new();
    let uri = temp_uri("format_a_sql_document_via_workspace.sql");
    let text = "select   a,b   from t\n";

    workspace
        .open_document(text_document(uri.clone(), "sql", text))
        .await
        .unwrap();

    let range = Range {
        start: Position::new(0, 0),
        end: Position::new(1, 0),
    };

    let edits = workspace
        .format(&uri, range, formatting_options())
        .await
        .expect("format should not error on a psql document")
        .expect("a well-formed query should produce an edit");

    assert_eq!(edits.len(), 1);
    assert_eq!(edits[0].new_text, "select a, b from t");
}

#[tokio::test]
async fn change_document_updates_a_sql_document_in_place() {
    let workspace = Workspace::new();
    let uri = temp_uri("change_document_updates_a_sql_document_in_place.sql");

    workspace
        .open_document(text_document(uri.clone(), "sql", "select a from t;\n"))
        .await
        .unwrap();

    let changed_text = "select   a   from   t;\n";
    workspace
        .change_document(text_document(uri.clone(), "sql", changed_text))
        .await
        .expect("change_document should not error on a psql document");

    let range = whole_document_range(changed_text);

    let edits = workspace
        .format(&uri, range, formatting_options())
        .await
        .expect("format should not error")
        .expect("should format the *updated* text");

    // Proves change_document actually replaced the tracked document
    // (not just re-inserted the original): the messy spacing from the
    // change_document call is what gets normalized here.
    assert_eq!(edits[0].new_text, "select a from t;");
}
