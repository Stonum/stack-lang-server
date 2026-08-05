//! Mirror of `workspace_sql.rs`, but for mlang (`.prg`) documents -- the
//! "home" language, so these are mostly regression coverage (make sure the
//! multi-language refactor didn't break the existing working path) rather
//! than fixing new bugs. Same rule: everything works off in-memory
//! `TextDocumentItem::text`, nothing read from disk.

mod common;

use common::{formatting_options, temp_uri, text_document, whole_document_range};
use stack_lang_server::workspace::Workspace;
use tower_lsp::lsp_types::{Position, Range};

#[tokio::test]
async fn opens_an_mlang_document_via_workspace() {
    let workspace = Workspace::new();
    let uri = temp_uri("opens_an_mlang_document_via_workspace.prg");

    let diagnostics = workspace
        .open_document(text_document(uri, "mlang", "var a = 1;\n"))
        .await
        .expect("open_document should not error");

    assert!(diagnostics.is_empty());
}

#[tokio::test]
async fn opening_malformed_mlang_reports_diagnostics_not_a_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("opening_malformed_mlang_reports_diagnostics_not_a_panic.prg");

    let diagnostics = workspace
        .open_document(text_document(uri, "mlang", "var a = ;;;\n"))
        .await
        .expect("open_document should not error, even for invalid mlang");

    assert!(!diagnostics.is_empty());
    assert_eq!(diagnostics[0].source.as_deref(), Some("mlang-parser"));
}

#[tokio::test]
async fn hover_on_an_mlang_document_does_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("hover_on_an_mlang_document_does_not_panic.prg");

    workspace
        .open_document(text_document(uri.clone(), "mlang", "var a = 1;\n"))
        .await
        .unwrap();

    let hover = workspace.hover(&uri, Position::new(0, 4)).await;

    assert!(hover.is_ok());
}

#[tokio::test]
async fn completion_on_an_mlang_document_does_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("completion_on_an_mlang_document_does_not_panic.prg");

    workspace
        .open_document(text_document(uri.clone(), "mlang", "var a = 1;\n"))
        .await
        .unwrap();

    let completion = workspace.completion(&uri, Position::new(0, 4)).await;

    assert!(completion.is_ok());
}

#[tokio::test]
async fn signature_help_on_an_mlang_document_does_not_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("signature_help_on_an_mlang_document_does_not_panic.prg");

    workspace
        .open_document(text_document(uri.clone(), "mlang", "var a = 1;\n"))
        .await
        .unwrap();

    let signature_help = workspace.signature_help(&uri, Position::new(0, 4)).await;

    assert!(signature_help.is_ok());
}

#[tokio::test]
async fn semantic_tokens_on_an_mlang_document_returns_tokens() {
    let workspace = Workspace::new();
    let uri = temp_uri("semantic_tokens_on_an_mlang_document_returns_tokens.prg");

    workspace
        .open_document(text_document(uri.clone(), "mlang", "var a = 1;\n"))
        .await
        .unwrap();

    let tokens = workspace
        .semantic_tokens(&uri, None)
        .await
        .expect("semantic_tokens should not error on an mlang document");

    // Unlike sql (no mlang tree at all -- always None), an mlang document
    // is exactly what this feature is for.
    assert!(tokens.is_some());
}

#[tokio::test]
async fn format_an_mlang_document_via_workspace() {
    let workspace = Workspace::new();
    let uri = temp_uri("format_an_mlang_document_via_workspace.prg");
    let text = "var   a=1;\n";

    workspace
        .open_document(text_document(uri.clone(), "mlang", text))
        .await
        .unwrap();

    let range = Range {
        start: Position::new(0, 0),
        end: Position::new(1, 0),
    };

    let edits = workspace
        .format(&uri, range, formatting_options())
        .await
        .expect("format should not error on an mlang document")
        .expect("a well-formed statement should produce an edit");

    assert_eq!(edits.len(), 1);
    assert_eq!(edits[0].new_text, "var a = 1;");
}

#[tokio::test]
async fn change_document_updates_an_mlang_document_in_place() {
    let workspace = Workspace::new();
    let uri = temp_uri("change_document_updates_an_mlang_document_in_place.prg");

    workspace
        .open_document(text_document(uri.clone(), "mlang", "var a = 1;\n"))
        .await
        .unwrap();

    let changed_text = "var   a   =   1;\n";
    workspace
        .change_document(text_document(uri.clone(), "mlang", changed_text))
        .await
        .expect("change_document should not error on an mlang document");

    let range = whole_document_range(changed_text);

    let edits = workspace
        .format(&uri, range, formatting_options())
        .await
        .expect("format should not error")
        .expect("should format the *updated* text");

    assert_eq!(edits[0].new_text, "var a = 1;");
}
