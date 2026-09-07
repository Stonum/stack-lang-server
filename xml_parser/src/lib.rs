//! XML parser for Stack `.rx` resources and `.xdic` dictionaries.
//!
//! Built on the same `biome_rowan` / `biome_parser` toolkit as `sql_parser`
//! and `mlang_parser`.

use std::marker::PhantomData;

use biome_parser::{
    AnyParse, ParserContext, ParserContextCheckpoint, event::Event, prelude::*,
    tree_sink::LosslessTreeSink,
};
use biome_rowan::{AstNode, NodeCache};
use xml_factory::XmlSyntaxFactory;
use xml_syntax::{XmlFileSource, XmlLanguage, XmlRoot, XmlSyntaxKind, XmlSyntaxNode};

use crate::token_source::{XmlTokenSource, XmlTokenSourceCheckpoint};

mod lexer;
mod syntax;
mod token_source;

pub use biome_parser::prelude::ParseDiagnostic;

pub type XmlLosslessTreeSink<'source> = LosslessTreeSink<'source, XmlLanguage, XmlSyntaxFactory>;

pub struct XmlParser<'source> {
    context: ParserContext<XmlSyntaxKind>,
    source: XmlTokenSource<'source>,
}

impl<'source> XmlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: XmlTokenSource::from_str(source),
        }
    }

    #[inline]
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut XmlParser) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);
        result
    }

    pub fn checkpoint(&self) -> XmlParserCheckpoint {
        XmlParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: XmlParserCheckpoint) {
        let XmlParserCheckpoint { context, source } = checkpoint;
        self.context.rewind(context);
        self.source.rewind(source);
    }

    pub fn finish(self) -> (Vec<Event<XmlSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
        let (trivia, _) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();
        (events, parse_diagnostics, trivia)
    }
}

impl<'source> Parser for XmlParser<'source> {
    type Kind = XmlSyntaxKind;
    type Source = XmlTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}

pub struct XmlParserCheckpoint {
    pub context: ParserContextCheckpoint,
    pub source: XmlTokenSourceCheckpoint,
}

#[derive(Debug)]
pub struct Parse<T> {
    root: XmlSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: XmlSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = XmlLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    pub fn syntax(&self) -> XmlSyntaxNode {
        self.root.clone()
    }

    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        self.errors.as_slice()
    }

    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.errors
    }

    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T: AstNode<Language = XmlLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    pub fn ok(self) -> Result<T, Vec<ParseDiagnostic>> {
        if !self.errors.iter().any(|d| d.is_error()) {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl<T> From<Parse<T>> for AnyParse {
    fn from(parse: Parse<T>) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        Self::new(
            // SAFETY: the parser always returns a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

pub fn parse(text: &str) -> Parse<XmlRoot> {
    let mut cache = NodeCache::default();
    parse_with_cache(text, &mut cache)
}

/// Convenience wrapper accepting an [XmlFileSource]; the source kind does not
/// change parsing today but keeps call sites (LSP dispatch, tests) uniform
/// with `sql_parser` / `mlang_parser`.
pub fn parse_xml(text: &str, _source_type: XmlFileSource) -> Parse<XmlRoot> {
    parse(text)
}

fn parse_with_cache(text: &str, cache: &mut NodeCache) -> Parse<XmlRoot> {
    let (events, errors, tokens) = parse_common(text);
    let mut tree_sink = XmlLosslessTreeSink::with_cache(text, &tokens, cache);
    biome_parser::event::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new(green, parse_errors)
}

fn parse_common(text: &str) -> (Vec<Event<XmlSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut p = XmlParser::new(text);
    syntax::parse_root(&mut p);
    p.finish()
}
