//! Shared helpers for `Workspace` end-to-end tests. Named `common/mod.rs`
//! (not a bare `tests/common.rs`) so Cargo doesn't pick it up as its own
//! standalone test binary -- same convention already used for
//! `sql_parser`/`mlang_formatter`'s own `tests/helper/mod.rs`.
//!
//! Each `tests/*.rs` file compiles `mod common;` as its own separate crate,
//! so a helper unused by one particular test file (but used by others)
//! would otherwise warn there -- hence the blanket allow.
#![allow(dead_code)]

use std::fs;

use tower_lsp::lsp_types::{FormattingOptions, Position, Range, TextDocumentItem, Url};

/// A real on-disk workspace dir, removed on drop.
pub struct TempDir(pub std::path::PathBuf);

impl TempDir {
    pub fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("sls_{}_{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp workspace dir");
        TempDir(dir)
    }

    /// `name` may contain subdirectories.
    pub fn write(&self, name: &str, contents: &str) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).expect("create temp workspace subdir");
        fs::write(path, contents).expect("write temp workspace file");
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A `file://` URI under the OS temp dir with the given file name --
/// doesn't need to exist on disk, since `Workspace`'s document-open path
/// only ever reads content from `TextDocumentItem::text`, never from the
/// path itself. Only `Url::to_file_path()`'s extension matters for
/// language routing.
pub fn temp_uri(name: &str) -> Url {
    let path = std::env::temp_dir().join(name);
    Url::from_file_path(path).expect("valid file path")
}

pub fn text_document(uri: Url, language_id: &str, text: &str) -> TextDocumentItem {
    TextDocumentItem {
        uri,
        language_id: language_id.to_string(),
        version: 1,
        text: text.to_string(),
    }
}

pub fn formatting_options() -> FormattingOptions {
    FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        properties: Default::default(),
        trim_trailing_whitespace: None,
        insert_final_newline: None,
        trim_final_newlines: None,
    }
}

pub fn whole_document_range(text: &str) -> Range {
    let lines = text.lines().count() as u32;
    Range {
        start: Position::new(0, 0),
        end: Position::new(lines, 0),
    }
}
