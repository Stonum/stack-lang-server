//! Files whose extension isn't recognized as either mlang or psql (e.g. a
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
