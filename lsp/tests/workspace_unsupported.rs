//! Files whose extension isn't recognized as either mlang or sql (e.g. a
//! plain `.txt`, or no extension at all). The extension is the *only*
//! thing `CurrentDocument::new` needs to route a document, so these must
//! fail cleanly with a `WorkspaceError` -- never panic -- both when opening
//! and when any other request lands on a URI that was never (successfully)
//! opened.

mod common;

use common::{temp_uri, text_document};
use stack_lang_server::workspace::Workspace;
use tower_lsp::lsp_types::Position;

#[tokio::test]
async fn opening_a_txt_document_fails_cleanly() {
    let workspace = Workspace::new();
    let uri = temp_uri("opening_a_txt_document_fails_cleanly.txt");

    let result = workspace
        .open_document(text_document(uri, "plaintext", "just some notes\n"))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn opening_a_document_with_no_extension_fails_cleanly() {
    let workspace = Workspace::new();
    let uri = temp_uri("opening_a_document_with_no_extension_fails_cleanly");

    let result = workspace
        .open_document(text_document(uri, "plaintext", "just some notes\n"))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn hover_on_a_never_opened_txt_document_fails_cleanly_not_a_panic() {
    let workspace = Workspace::new();
    let uri = temp_uri("hover_on_a_never_opened_txt_document_fails_cleanly_not_a_panic.txt");

    // Never opened via open_document -- hover falls back to
    // get_opened_document's "read from disk" path, which fails the same
    // extension check before ever touching the filesystem.
    let result = workspace.hover(&uri, Position::new(0, 0)).await;

    assert!(result.is_err());
}

// The LSP layer (`lsp/src/main.rs`) uses `is_unsupported_document` to tell
// "editor assigned an unrecognized language to this file" apart from a
// genuine server malfunction, so it can respond with an empty result
// instead of logging an error and returning `Internal error` to the
// client on every request. These are the two error paths above, so make
// sure both are classified as unsupported-document, not a real failure.
#[tokio::test]
async fn missing_extension_error_is_classified_as_unsupported_document() {
    let workspace = Workspace::new();
    let uri = temp_uri("missing_extension_error_is_classified_as_unsupported_document");

    let error = workspace
        .open_document(text_document(uri, "plaintext", "just some notes\n"))
        .await
        .expect_err("no-extension document should fail to open");

    assert!(error.is_unsupported_document());
}

#[tokio::test]
async fn unknown_extension_error_is_classified_as_unsupported_document() {
    let workspace = Workspace::new();
    let uri = temp_uri("unknown_extension_error_is_classified_as_unsupported_document.txt");

    let error = workspace
        .open_document(text_document(uri, "plaintext", "just some notes\n"))
        .await
        .expect_err(".txt document should fail to open");

    assert!(error.is_unsupported_document());
}
