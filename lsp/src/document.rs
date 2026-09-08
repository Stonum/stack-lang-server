use biome_diagnostics::diagnostic::Diagnostic as _;
use line_index::{LineColRange, LineIndex};

use mlang_parser::ParseDiagnostic;
use mlang_semantic::{AnyMDefinition, SemanticModel, semantics};
use mlang_syntax::{
    FileSourceError, MFileSource, MLanguage, MSyntaxNode, SendNode, SyntaxNode, TextRange,
};

use sql_syntax::{SqlDialect, SqlFileSource, SqlLanguage, SqlSyntaxNode};
use xml_syntax::{XmlFileSource, XmlLanguage, XmlSyntaxNode, XmlVariant};

use std::{
    any::type_name,
    path::{Path, PathBuf},
};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DocumentSymbol, NumberOrString, Position, Range, SymbolKind,
    Url,
};

/// What kind of Stack document this is, derived from the file extension.
///
/// This is the single classification the LSP layer works with; the
/// per-language file sources ([MFileSource], [SqlFileSource],
/// [XmlFileSource]) are re-derived from it through the accessors below.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DocumentKind {
    /// mlang module — `.prg`
    Module,
    /// mlang event handler — `.hdl`
    Handler,
    /// mlang report — `.rpt*` / `.pa*`
    Report,
    /// SQL script — `.sql`
    Sql,
    /// XML resource — `.rx`, `.rx_api`, `.rxz`, …
    Resource,
    /// XML dictionary — `.xdic`
    Dictionary,
    /// Plain XML — `.xml`
    Xml,
}

impl DocumentKind {
    /// Classify a document by its path. `Err` means the extension is not one
    /// we handle.
    pub fn from_path(path: &Path) -> Result<Self, FileSourceError> {
        if SqlFileSource::try_from(path).is_ok() {
            return Ok(Self::Sql);
        }
        if let Ok(xml) = XmlFileSource::try_from(path) {
            return Ok(Self::from_xml(xml));
        }
        Ok(Self::from_mlang(MFileSource::try_from(path)?))
    }

    fn from_mlang(source: MFileSource) -> Self {
        if source.is_handler() {
            Self::Handler
        } else if source.is_report() {
            Self::Report
        } else {
            Self::Module
        }
    }

    fn from_xml(source: XmlFileSource) -> Self {
        match source.variant() {
            XmlVariant::Resource => Self::Resource,
            XmlVariant::Dictionary => Self::Dictionary,
            XmlVariant::Plain => Self::Xml,
        }
    }

    pub fn is_mlang(&self) -> bool {
        matches!(self, Self::Module | Self::Handler | Self::Report)
    }

    pub fn is_sql(&self) -> bool {
        matches!(self, Self::Sql)
    }

    pub fn is_xml(&self) -> bool {
        matches!(self, Self::Resource | Self::Dictionary | Self::Xml)
    }

    pub fn mlang_file_source(&self) -> Option<MFileSource> {
        Some(match self {
            Self::Module => MFileSource::module(),
            Self::Handler => MFileSource::handler(),
            Self::Report => MFileSource::report(),
            _ => return None,
        })
    }

    pub fn sql_file_source(&self) -> Option<SqlFileSource> {
        matches!(self, Self::Sql).then(|| {
            SqlFileSource::script()
                .with_dialect(SqlDialect::Postgres)
                .with_mlang_extension(true)
        })
    }

    pub fn xml_file_source(&self) -> Option<XmlFileSource> {
        Some(match self {
            Self::Resource => XmlFileSource::resource(),
            Self::Dictionary => XmlFileSource::dictionary(),
            Self::Xml => XmlFileSource::plain(),
            _ => return None,
        })
    }

    /// Prefix for the `source` field of the diagnostics this document
    /// produces (`mlang-parser`, `sql-parser`, …).
    pub fn diagnostic_source(&self) -> &'static str {
        if self.is_sql() {
            "sql"
        } else if self.is_xml() {
            "xml"
        } else {
            "mlang"
        }
    }
}

pub struct CurrentDocument {
    uri: Url,
    root: SendNode,
    kind: DocumentKind,
    line_index: LineIndex,
    semantics: Option<SemanticModel>,
    parse_diagnostics: Vec<ParseDiagnostic>,
}

impl CurrentDocument {
    pub fn new(uri: Url, path: &Path, text: &str) -> Result<CurrentDocument, FileSourceError> {
        let kind = DocumentKind::from_path(path)?;

        Ok(match kind {
            DocumentKind::Sql => {
                CurrentDocument::new_sql(uri, text, kind.sql_file_source().unwrap())
            }
            DocumentKind::Resource | DocumentKind::Dictionary | DocumentKind::Xml => {
                CurrentDocument::new_xml(uri, text, kind.xml_file_source().unwrap())
            }
            DocumentKind::Module | DocumentKind::Handler | DocumentKind::Report => {
                CurrentDocument::new_mlang(uri, text, kind.mlang_file_source().unwrap())
            }
        })
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
            kind: DocumentKind::from_mlang(file_source),
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
            kind: DocumentKind::Sql,
            semantics: None,
            line_index,
            parse_diagnostics,
        }
    }

    pub fn new_xml(uri: Url, text: &str, file_source: XmlFileSource) -> CurrentDocument {
        let parsed = xml_parser::parse(text);
        let diagnostics = parsed.diagnostics();

        Self::from_xml_root(uri, text, file_source, parsed.syntax(), diagnostics)
    }

    pub fn from_xml_root(
        uri: Url,
        text: &str,
        file_source: XmlFileSource,
        root: XmlSyntaxNode,
        diagnostics: &[ParseDiagnostic],
    ) -> CurrentDocument {
        let root = root.as_send().unwrap_or_else(|| {
            panic!(
                "could not upcast root node from language {}",
                type_name::<XmlLanguage>()
            )
        });
        let line_index = LineIndex::new(text);
        let parse_diagnostics = diagnostics.to_vec();

        CurrentDocument {
            uri,
            root,
            kind: DocumentKind::from_xml(file_source),
            semantics: None,
            line_index,
            parse_diagnostics,
        }
    }

    pub fn kind(&self) -> DocumentKind {
        self.kind
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

    pub fn xml_syntax(&self) -> Option<SyntaxNode<XmlLanguage>> {
        self.root.clone().into_node()
    }

    pub fn xml_file_source(&self) -> Option<XmlFileSource> {
        self.kind.xml_file_source()
    }

    /// A nested outline of the document, for `textDocument/documentSymbol`.
    /// `None` for non-XML documents (those go through `definitions()`).
    pub fn xml_document_symbols(&self) -> Option<Vec<DocumentSymbol>> {
        let root = self.xml_syntax()?;
        let symbols = xml_semantic::document_symbols(&root);
        Some(to_lsp_symbols(&self.line_index, &symbols))
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

        let source = self.kind.diagnostic_source();

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

        let from_mlang_lint = semantic_lint.iter().filter_map(|diagnostic| {
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

        let from_xml_lint = self.xml_lint().into_iter().filter_map(|diagnostic| {
            let range = to_lsp_range(line_index, diagnostic.range)?;
            let severity = match diagnostic.severity {
                xml_lint::Severity::Error => DiagnosticSeverity::ERROR,
                xml_lint::Severity::Warning => DiagnosticSeverity::WARNING,
            };
            Some(Diagnostic::new(
                range,
                Some(severity),
                Some(NumberOrString::String(diagnostic.code.to_string())),
                Some(format!("{source}-lint")),
                diagnostic.message,
                None,
                None,
            ))
        });

        from_parser
            .chain(from_mlang_lint)
            .chain(from_xml_lint)
            .collect()
    }

    fn xml_lint(&self) -> Vec<xml_lint::Diagnostic> {
        match self.xml_syntax() {
            Some(root) => xml_lint::syntax_diagnostics(&root),
            None => Vec::new(),
        }
    }
}

fn to_lsp_range(line_index: &LineIndex, text_range: TextRange) -> Option<Range> {
    let LineColRange { start, end } = line_index.line_col_range(text_range)?;
    Some(Range::new(
        Position::new(start.line, start.col),
        Position::new(end.line, end.col),
    ))
}

fn to_lsp_symbols(
    line_index: &LineIndex,
    symbols: &[xml_semantic::DocumentSymbol],
) -> Vec<DocumentSymbol> {
    symbols
        .iter()
        .filter_map(|symbol| {
            let range = to_lsp_range(line_index, symbol.range)?;
            let selection_range = to_lsp_range(line_index, symbol.selection_range)?;
            let kind = match symbol.kind {
                xml_semantic::SymbolKind::Container => SymbolKind::OBJECT,
                xml_semantic::SymbolKind::Leaf => SymbolKind::FIELD,
            };

            #[allow(deprecated)]
            Some(DocumentSymbol {
                name: symbol.name.clone(),
                detail: symbol.detail.clone(),
                kind,
                tags: None,
                deprecated: None,
                range,
                selection_range,
                children: Some(to_lsp_symbols(line_index, &symbol.children)),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind_of(name: &str) -> DocumentKind {
        DocumentKind::from_path(&std::path::PathBuf::from(name)).expect("known extension")
    }

    #[test]
    fn classifies_documents_by_extension() {
        assert_eq!(kind_of("a.prg"), DocumentKind::Module);
        assert_eq!(kind_of("a.hdl"), DocumentKind::Handler);
        assert_eq!(kind_of("a.rpt"), DocumentKind::Report);
        assert_eq!(kind_of("a.sql"), DocumentKind::Sql);
        assert_eq!(kind_of("a.xdic"), DocumentKind::Dictionary);
        assert_eq!(kind_of("a.xml"), DocumentKind::Xml);
        for name in ["a.rx", "a.rx_api", "a.rxz"] {
            assert_eq!(kind_of(name), DocumentKind::Resource, "{name}");
        }
    }

    #[test]
    fn accessors_round_trip_to_the_right_file_source() {
        assert_eq!(
            kind_of("a.xdic").xml_file_source().map(|s| s.variant()),
            Some(XmlVariant::Dictionary)
        );
        assert!(kind_of("a.hdl").mlang_file_source().unwrap().is_handler());
        assert!(kind_of("a.sql").sql_file_source().is_some());
        assert!(kind_of("a.rx").mlang_file_source().is_none());
        assert!(kind_of("a.prg").xml_file_source().is_none());
    }

    #[test]
    fn opened_xml_document_reports_its_kind() {
        let uri = Url::parse("file:///a.xdic").unwrap();
        let doc =
            CurrentDocument::new(uri, &std::path::PathBuf::from("a.xdic"), "<root/>").unwrap();
        assert_eq!(doc.kind(), DocumentKind::Dictionary);
        assert_eq!(
            doc.xml_file_source().map(|s| s.variant()),
            Some(XmlVariant::Dictionary)
        );
    }

    #[test]
    fn xml_document_symbols_form_a_nested_outline() {
        let uri = Url::parse("file:///a.rx").unwrap();
        let text = "<root>\n  <group name=\"g\">\n    <item/>\n  </group>\n</root>\n";
        let doc = CurrentDocument::new(uri, &std::path::PathBuf::from("a.rx"), text).unwrap();

        let symbols = doc.xml_document_symbols().expect("xml document");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "root");

        let group = &symbols[0].children.as_ref().unwrap()[0];
        assert_eq!(group.name, "g");
        assert_eq!(group.detail.as_deref(), Some("group"));
        assert_eq!(group.children.as_ref().unwrap()[0].name, "item");
    }

    #[test]
    fn non_xml_document_has_no_xml_symbols() {
        let uri = Url::parse("file:///a.sql").unwrap();
        let doc =
            CurrentDocument::new(uri, &std::path::PathBuf::from("a.sql"), "select 1").unwrap();
        assert!(doc.xml_document_symbols().is_none());
    }

    #[test]
    fn xml_lint_flags_a_duplicate_attribute() {
        let uri = Url::parse("file:///a.rx").unwrap();
        let doc = CurrentDocument::new(
            uri,
            &std::path::PathBuf::from("a.rx"),
            r#"<root a="1" a="2"/>"#,
        )
        .unwrap();

        let diagnostics = doc.diagnostics(&[]);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].source.as_deref(), Some("xml-lint"));
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    }
}
