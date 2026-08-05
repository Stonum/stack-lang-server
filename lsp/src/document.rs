use biome_diagnostics::diagnostic::Diagnostic as _;
use line_index::{LineColRange, LineIndex};

use mlang_parser::ParseDiagnostic;
use mlang_semantic::{AnyMDefinition, SemanticModel, semantics};
use mlang_syntax::{
    FileSourceError, MFileSource, MLanguage, MSyntaxNode, SendNode, SyntaxNode, TextRange,
};

use sql_syntax::{SqlDialect, SqlFileSource, SqlLanguage, SqlSyntaxNode};

use std::{
    any::type_name,
    path::{Path, PathBuf},
};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DocumentLanguage {
    Mlang,
    Sql,
}

pub struct CurrentDocument {
    uri: Url,
    root: SendNode,
    language: DocumentLanguage,
    line_index: LineIndex,
    semantics: Option<SemanticModel>,
    parse_diagnostics: Vec<ParseDiagnostic>,
}

impl CurrentDocument {
    pub fn new(uri: Url, path: &Path, text: &str) -> Result<CurrentDocument, FileSourceError> {
        if let Ok(file_source) = SqlFileSource::try_from(path) {
            let file_source = file_source.with_dialect(SqlDialect::Mlang);
            return Ok(CurrentDocument::new_sql(uri, text, file_source));
        }

        let file_source = MFileSource::try_from(path)?;
        Ok(CurrentDocument::new_mlang(uri, text, file_source))
    }

    pub fn new_mlang(uri: Url, text: &str, file_source: MFileSource) -> CurrentDocument {
        let parsed = mlang_parser::parse(text, file_source);
        let diagnostics = parsed.diagnostics();

        Self::from_mlang_root(uri, file_source, text, parsed.syntax(), diagnostics)
    }

    pub fn from_mlang_root(
        uri: Url,
        file_source: MFileSource,
        text: &str,
        root: MSyntaxNode,
        diagnostics: &[ParseDiagnostic],
    ) -> CurrentDocument {
        let semantics = Some(semantics(text, root.clone(), file_source));
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
            language: DocumentLanguage::Mlang,
            semantics,
            line_index,
            parse_diagnostics,
        }
    }

    pub fn new_sql(uri: Url, text: &str, file_source: SqlFileSource) -> CurrentDocument {
        let parsed = sql_parser::parse(text, file_source);
        let diagnostics = parsed.diagnostics();

        Self::from_sql_root(uri, text, parsed.syntax(), diagnostics)
    }

    pub fn from_sql_root(
        uri: Url,
        text: &str,
        root: SqlSyntaxNode,
        diagnostics: &[ParseDiagnostic],
    ) -> CurrentDocument {
        let root = root.as_send().unwrap_or_else(|| {
            panic!(
                "could not upcast root node from language {}",
                type_name::<SqlLanguage>()
            )
        });
        let line_index = LineIndex::new(text);
        let parse_diagnostics = diagnostics.to_vec();

        CurrentDocument {
            uri,
            root,
            language: DocumentLanguage::Sql,
            semantics: None,
            line_index,
            parse_diagnostics,
        }
    }

    pub fn language(&self) -> DocumentLanguage {
        self.language
    }

    pub fn path(&self) -> PathBuf {
        self.uri.to_file_path().unwrap_or_default()
    }

    pub fn uri(&self) -> &Url {
        &self.uri
    }

    pub fn mlang_syntax(&self) -> Option<SyntaxNode<MLanguage>> {
        self.root.clone().into_node()
    }

    pub fn sql_syntax(&self) -> Option<SyntaxNode<SqlLanguage>> {
        self.root.clone().into_node()
    }

    pub fn definitions(&self) -> core::slice::Iter<'_, AnyMDefinition> {
        static EMPTY: &[AnyMDefinition] = &[];
        self.semantics
            .as_ref()
            .map_or_else(|| EMPTY.iter(), |semantics| semantics.definitions())
    }

    pub fn line_index(&self) -> &LineIndex {
        &self.line_index
    }

    pub fn diagnostics(&self, semantic_lint: &[mlang_lint::Diagnostic]) -> Vec<Diagnostic> {
        let line_index = &self.line_index;

        let source = match self.language {
            DocumentLanguage::Mlang => "mlang",
            DocumentLanguage::Sql => "sql",
        };

        let from_parser = self.parse_diagnostics.iter().filter_map(|error| {
            let text_range = error.location().span?;
            let range = to_lsp_range(line_index, text_range)?;
            Some(Diagnostic::new(
                range,
                Some(DiagnosticSeverity::ERROR),
                None,
                Some(format!("{source}-parser")),
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
                Some(format!("{source}-lint")),
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
