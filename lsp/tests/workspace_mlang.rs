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
async fn missing_closing_brace_is_pinpointed_at_the_edit_site() {
    let workspace = Workspace::new();
    let uri = temp_uri("missing_closing_brace_is_pinpointed_at_the_edit_site.prg");

    // The `}` for the inner `if` block (line 1) is missing. The parser eats the
    // outer block's `}` to close the `if`, then runs out of tokens at EOF -- the
    // raw diagnostic would collapse to a zero-width range on the last line. The
    // indentation heuristic must instead point at the inner `{`.
    let source = "func main() {\n    if (x) {\n        work();\n\n    println(\"after\");\n}\n";

    let diagnostics = workspace
        .open_document(text_document(uri, "mlang", source))
        .await
        .expect("open_document should not error");

    let unclosed = diagnostics
        .iter()
        .find(|d| d.message.contains("never closed"))
        .expect("expected an 'unclosed `{`' diagnostic");

    // Anchored on the `{` of `if (x) {` (line 1), not the trailing `}` line.
    assert_eq!(unclosed.range.start.line, 1);
    assert!(unclosed.range.end > unclosed.range.start);

    // The coarse end-of-file diagnostic is suppressed once we have the pinpoint.
    assert!(
        !diagnostics
            .iter()
            .any(|d| d.message.contains("Missing closing")),
        "coarse EOF diagnostic should be dropped"
    );
}

#[tokio::test]
async fn missing_closing_brace_highlights_the_whole_header_line() {
    let workspace = Workspace::new();
    let uri = temp_uri("missing_closing_brace_highlights_the_whole_header_line.prg");

    // K&R style: the `{` sits alone on its line, so the squiggle should land on
    // the `while (b)` header line above it and span the whole line, not a single
    // character.
    let source = "func main()\n{\n    while (b)\n    {\n        work();\n}\n";

    let diagnostics = workspace
        .open_document(text_document(uri, "mlang", source))
        .await
        .expect("open_document should not error");

    let unclosed = diagnostics
        .iter()
        .find(|d| d.message.contains("never closed"))
        .expect("expected an 'unclosed `{`' diagnostic");

    // `while (b)` is line 2 (0-based), indented 4.
    assert_eq!(unclosed.range.start.line, 2);
    assert_eq!(unclosed.range.end.line, 2);
    assert_eq!(unclosed.range.start.character, 4);
    assert!(
        unclosed.range.end.character - unclosed.range.start.character >= "while (b)".len() as u32
    );
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
