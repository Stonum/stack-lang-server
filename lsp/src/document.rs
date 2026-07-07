use biome_diagnostics::diagnostic::Diagnostic as _;
use line_index::{LineColRange, LineIndex};

use mlang_parser::{ParseDiagnostic, parse};
use mlang_semantic::{AnyMDefinition, SemanticModel, semantics};
use mlang_syntax::{MFileSource, MLanguage, MSyntaxNode, SendNode, SyntaxNode, TextRange};

use std::{any::type_name, path::PathBuf};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};

pub struct CurrentDocument {
    uri: Url,
    root: SendNode,
    line_index: LineIndex,
    semantics: SemanticModel,
    parse_diagnostics: Vec<ParseDiagnostic>,
}

impl CurrentDocument {
    pub fn new(uri: Url, text: &str, file_source: MFileSource) -> CurrentDocument {
        let parsed = parse(text, file_source);
        let diagnostics = parsed.diagnostics();

        Self::from_root(uri, file_source, text, parsed.syntax(), diagnostics)
    }

    pub fn from_root(
        uri: Url,
        file_source: MFileSource,
        text: &str,
        root: MSyntaxNode,
        diagnostics: &[ParseDiagnostic],
    ) -> CurrentDocument {
        let semantics = semantics(text, root.clone(), file_source);
        let root = root.as_send().unwrap_or_else(|| {
            panic!(
                "could not upcast root node from language {}",
                type_name::<MLanguage>()
            )
        });
        let line_index = LineIndex::new(text);
        let parse_diagnostics = diagnostics.to_vec();

        CurrentDocument {
            uri,
            root,
            semantics,
            line_index,
            parse_diagnostics,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.uri.to_file_path().unwrap_or_default()
    }

    pub fn uri(&self) -> &Url {
        &self.uri
    }

    pub fn syntax(&self) -> SyntaxNode<MLanguage> {
        self.root.clone().into_node().unwrap_or_else(|| {
            panic!(
                "could not downcast root node to language {}",
                type_name::<MLanguage>()
            )
        })
    }

    pub fn definitions(&self) -> core::slice::Iter<'_, AnyMDefinition> {
        self.semantics.definitions()
    }

    pub fn line_index(&self) -> &LineIndex {
        &self.line_index
    }

    pub fn diagnostics(&self, semantic_lint: &[mlang_lint::Diagnostic]) -> Vec<Diagnostic> {
        let line_index = &self.line_index;

        let from_parser = self.parse_diagnostics.iter().filter_map(|error| {
            let text_range = error.location().span?;
            let range = to_lsp_range(line_index, text_range)?;
            Some(Diagnostic::new(
                range,
                Some(DiagnosticSeverity::ERROR),
                None,
                Some("mlang-parser".to_string()),
                error.message.to_string(),
                None,
                None,
            ))
        });

        let from_lint = semantic_lint.iter().filter_map(|diagnostic| {
            let range = to_lsp_range(line_index, diagnostic.range)?;
            let severity = match diagnostic.severity {
                mlang_lint::Severity::Error => DiagnosticSeverity::ERROR,
                mlang_lint::Severity::Warning => DiagnosticSeverity::WARNING,
            };
            Some(Diagnostic::new(
                range,
                Some(severity),
                Some(NumberOrString::String(diagnostic.code.to_string())),
                Some("mlang-lint".to_string()),
                diagnostic.message.clone(),
                None,
                None,
            ))
        });

        from_parser.chain(from_lint).collect()
    }
}

fn to_lsp_range(line_index: &LineIndex, text_range: TextRange) -> Option<Range> {
    let LineColRange { start, end } = line_index.line_col_range(text_range)?;
    Some(Range::new(
        Position::new(start.line, start.col),
        Position::new(end.line, end.col),
    ))
}
