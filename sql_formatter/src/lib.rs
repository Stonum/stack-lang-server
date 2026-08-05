mod cst;
mod generated;
mod macros;
mod prelude;
mod rules;
mod sql;
mod syntax_rewriter;

pub(crate) mod comments;
pub(crate) mod context;

pub(crate) mod separated;
pub(crate) mod utils;

use biome_formatter::prelude::*;
use biome_formatter::{
    CstFormatContext, Format, FormatLanguage, TransformSourceMap, comments::Comments, write,
};
use sql_syntax::{AnySqlStatement, SqlLanguage, SqlSyntaxNode};

use biome_formatter::{Buffer, Formatted, Printed};
pub use biome_formatter::{IndentStyle, IndentWidth, LineWidth};
use biome_rowan::TextRange;
use biome_rowan::{AstNode, SyntaxNode};

use comments::SqlCommentStyle;
pub(crate) use context::SqlFormatContext;
pub use context::SqlFormatOptions;
use cst::FormatSqlSyntaxNode;

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: biome_formatter::Format<Context>
    where
        Self: 'a;

    /// Returns an object that is able to format this object.
    fn format(&self) -> Self::Format<'_>;
}

/// Implement [AsFormat] for references to types that implement [AsFormat].
impl<T, C> AsFormat<C> for &T
where
    T: AsFormat<C>,
{
    type Format<'a>
        = T::Format<'a>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        AsFormat::format(&**self)
    }
}

/// Implement [AsFormat] for [SyntaxResult] where `T` implements [AsFormat].
///
/// Useful to format mandatory AST fields without having to unwrap the value first.
impl<T, C> AsFormat<C> for biome_rowan::SyntaxResult<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = biome_rowan::SyntaxResult<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        match self {
            Ok(value) => Ok(value.format()),
            Err(err) => Err(*err),
        }
    }
}

/// Implement [AsFormat] for [Option] when `T` implements [AsFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, C> AsFormat<C> for Option<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = Option<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        self.as_ref().map(|value| value.format())
    }
}

/// Used to convert this object into an object that can be formatted.
///
/// The difference to [AsFormat] is that this trait takes ownership of `self`.
pub(crate) trait IntoFormat<Context> {
    type Format: biome_formatter::Format<Context>;

    fn into_format(self) -> Self::Format;
}

impl<T, Context> IntoFormat<Context> for biome_rowan::SyntaxResult<T>
where
    T: IntoFormat<Context>,
{
    type Format = biome_rowan::SyntaxResult<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Implement [IntoFormat] for [Option] when `T` implements [IntoFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, Context> IntoFormat<Context> for Option<T>
where
    T: IntoFormat<Context>,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting specific [Iterator] extensions
pub(crate) trait FormattedIterExt {
    /// Converts every item to an object that knows how to format it.
    fn formatted<Context>(self) -> FormattedIter<Self, Self::Item, Context>
    where
        Self: Iterator + Sized,
        Self::Item: IntoFormat<Context>,
    {
        FormattedIter {
            inner: self,
            options: std::marker::PhantomData,
        }
    }
}

impl<I> FormattedIterExt for I where I: std::iter::Iterator {}

pub(crate) struct FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
    options: std::marker::PhantomData<Context>,
}

impl<Iter, Item, Context> std::iter::Iterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
    Item: IntoFormat<Context>,
{
    type Item = Item::Format;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_format())
    }
}

impl<Iter, Item, Context> std::iter::FusedIterator for FormattedIter<Iter, Item, Context>
where
    Iter: std::iter::FusedIterator<Item = Item>,
    Item: IntoFormat<Context>,
{
}

impl<Iter, Item, Context> std::iter::ExactSizeIterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item> + std::iter::ExactSizeIterator,
    Item: IntoFormat<Context>,
{
}

pub(crate) type SqlFormatter<'buf> = Formatter<'buf, SqlFormatContext>;

/// Rule for formatting a Sql [AstNode].
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = SqlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut SqlFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node without comments. Ignores any suppression comments.
    fn fmt_node(&self, node: &N, f: &mut SqlFormatter) -> FormatResult<()> {
        let needs_parentheses = self.needs_parentheses(node);

        if needs_parentheses {
            write!(f, [text("(")])?;
        }

        self.fmt_fields(node, f)?;

        if needs_parentheses {
            write!(f, [text(")")])?;
        }

        Ok(())
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut SqlFormatter) -> FormatResult<()>;

    /// Returns whether the node requires parens.
    fn needs_parentheses(&self, item: &N) -> bool {
        let _ = item;
        false
    }

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &SqlFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut SqlFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut SqlFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut SqlFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting a bogus node.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = SqlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut SqlFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SqlFormatLanguage {
    options: SqlFormatOptions,
}
impl SqlFormatLanguage {
    pub fn new(options: SqlFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for SqlFormatLanguage {
    type SyntaxLanguage = SqlLanguage;
    type Context = SqlFormatContext;
    type FormatRule = FormatSqlSyntaxNode;

    fn transform(
        &self,
        root: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> Option<(SyntaxNode<Self::SyntaxLanguage>, TransformSourceMap)> {
        // The source-map offset math `syntax_rewriter::transform` relies on
        // (via `TransformSourceMapBuilder::with_offset`) only holds when
        // `root` is the true document root (offset 0). Formatting an
        // arbitrary sub-node in isolation -- as the `assert_fmt_node!` test
        // helper does, to exercise one node kind's formatting without an
        // unimplemented ancestor swallowing it into `format_verbatim_node`
        // -- hits a `debug_assert` bug in `biome_formatter`'s
        // `TransformSourceMap` when the offset is non-zero (its bounds
        // check subtracts the offset instead of adding it, which happens
        // to be a no-op, and so goes unnoticed, when the offset is always
        // 0 -- the only case the upstream crate's own tests exercise).
        // Skipping the transform for a non-root node only turns off
        // parenthesis normalization for that narrow sub-node-only
        // formatting path; real whole-file formatting (`format_node`
        // called on the actual file root, as production code and
        // `assert_fmt!`/`assert_fmt_eq!` do) is unaffected.
        if root.parent().is_some() {
            return None;
        }
        Some(syntax_rewriter::transform(root.clone()))
    }

    fn is_range_formatting_node(&self, node: &SqlSyntaxNode) -> bool {
        AnySqlStatement::can_cast(node.kind())
    }

    fn options(&self) -> &SqlFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        root: &SqlSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &SqlCommentStyle, source_map.as_ref());
        SqlFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [SqlFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: SqlFormatOptions,
    root: &SqlSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, SqlFormatLanguage::new(options))
}

/// Formats a SQL file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: SqlFormatOptions,
    root: &SqlSyntaxNode,
) -> FormatResult<Formatted<SqlFormatContext>> {
    biome_formatter::format_node(root, SqlFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [SqlFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_sub_tree(options: SqlFormatOptions, root: &SqlSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, SqlFormatLanguage::new(options))
}
