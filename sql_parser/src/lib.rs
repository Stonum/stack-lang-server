use std::marker::PhantomData;

use biome_parser::{
    AnyParse, ParserContext, ParserContextCheckpoint, event::Event, prelude::*,
    tree_sink::LosslessTreeSink,
};
use biome_rowan::{AstNode, NodeCache};
use sql_factory::SqlSyntaxFactory;
use sql_syntax::{SqlFileSource, SqlLanguage, SqlRoot, SqlSyntaxKind, SqlSyntaxNode};

use crate::{
    lexer::SqlReLexContext,
    token_source::{SqlTokenSource, SqlTokenSourceCheckpoint},
};

mod lexer;
mod syntax_rules;
mod token_source;

/// Optional syntax capabilities a piece of the grammar can require, checked
/// uniformly via [SyntaxFeature] -- mirrors `biome_js_parser`'s own
/// `JsSyntaxFeature` (dialect/extension gates as a small enum + a shared
/// `is_supported` check, rather than each parse site re-deriving its own
/// `p.source_type()...` condition). Not yet applied to gate any specific
/// node: `Postgres`/`Mssql` exist so a future dialect-specific grammar rule
/// (e.g. `TOP` under `Mssql`, `ON CONFLICT` restricted to `Postgres`) has
/// something to check against, via [SyntaxFeature::exclusive_syntax]/
/// [SyntaxFeature::parse_exclusive_syntax] -- both let a node parse the
/// same way regardless of dialect and only turn it into a diagnostic +
/// bogus node afterward if the active dialect doesn't support it, so
/// grammar rules themselves don't need per-dialect branches.
pub enum SqlSyntaxFeature {
    Postgres,
    Mssql,
    Mlang,
}

impl SyntaxFeature for SqlSyntaxFeature {
    type Parser<'source> = SqlParser<'source>;

    fn is_supported(&self, p: &SqlParser) -> bool {
        match self {
            Self::Postgres => p.source_type().dialect().is_postgres(),
            Self::Mssql => p.source_type().dialect().is_mssql(),
            Self::Mlang => p.source_type().has_mlang_extension(),
        }
    }
}

pub type SqlLosslessTreeSink<'source> = LosslessTreeSink<'source, SqlLanguage, SqlSyntaxFactory>;
pub struct SqlParser<'source> {
    pub source_type: SqlFileSource,
    context: ParserContext<SqlSyntaxKind>,
    source: SqlTokenSource<'source>,
}

impl<'source> SqlParser<'source> {
    pub fn new(source: &'source str, source_type: SqlFileSource) -> Self {
        Self {
            source_type,
            context: ParserContext::default(),
            source: SqlTokenSource::from_str(source, source_type),
        }
    }

    pub fn source_type(&self) -> SqlFileSource {
        self.source_type
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: SqlReLexContext) -> SqlSyntaxKind {
        self.source_mut().re_lex(context)
    }

    /// Stores the parser state and position before calling the function and restores the state
    /// and position before returning.
    ///
    /// Useful in situation where the parser must advance a few tokens to determine whatever a syntax is
    /// of one or the other kind.
    #[inline]
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut SqlParser) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);

        result
    }

    pub fn checkpoint(&self) -> SqlParserCheckpoint {
        SqlParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: SqlParserCheckpoint) {
        let SqlParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
    }

    pub fn finish(self) -> (Vec<Event<SqlSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
        let (trivia, _) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        (events, parse_diagnostics, trivia)
    }
}

impl<'source> Parser for SqlParser<'source> {
    type Kind = SqlSyntaxKind;
    type Source = SqlTokenSource<'source>;

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

pub struct SqlParserCheckpoint {
    pub context: ParserContextCheckpoint,
    pub source: SqlTokenSourceCheckpoint,
}

#[derive(Debug)]
pub struct Parse<T> {
    root: SqlSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: SqlSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = SqlLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> SqlSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        self.errors.as_slice()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T: AstNode<Language = SqlLanguage>> Parse<T> {
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

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
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
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

pub fn parse(text: &str, source_type: SqlFileSource) -> Parse<SqlRoot> {
    let mut cache = NodeCache::default();
    parse_with_cache(text, source_type, &mut cache)
}

fn parse_with_cache(
    text: &str,
    source_type: SqlFileSource,
    cache: &mut NodeCache,
) -> Parse<SqlRoot> {
    let (events, errors, tokens) = parse_common(text, source_type);
    let mut tree_sink = SqlLosslessTreeSink::with_cache(text, &tokens, cache);
    biome_parser::event::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new(green, parse_errors)
}

fn parse_common(
    text: &str,
    source_type: SqlFileSource,
) -> (Vec<Event<SqlSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut p = SqlParser::new(text, source_type);
    syntax_rules::program::parse(&mut p);

    let (events, errors, trivia) = p.finish();

    (events, errors, trivia)
}
