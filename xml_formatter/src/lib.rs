//! XML formatter for Stack `.rx` resources and `.xdic` dictionaries.
//!
//! Built on `biome_formatter`, mirroring `sql_formatter`.

mod cst;
mod generated;
mod macros;
mod prelude;
mod rules;
mod xml;

pub(crate) mod comments;
pub(crate) mod context;

use biome_formatter::prelude::*;
use biome_formatter::{
    Buffer, CstFormatContext, Format, FormatLanguage, Formatted, Printed, TransformSourceMap,
    comments::Comments, write,
};
use biome_rowan::{AstNode, TextRange};
use xml_syntax::{XmlLanguage, XmlSyntaxNode};

use comments::XmlCommentStyle;
pub(crate) use context::XmlFormatContext;
pub use context::{SelfClosingSpacing, XmlFormatOptions};
use cst::FormatXmlSyntaxNode;

pub use biome_formatter::{IndentStyle, IndentWidth, LineWidth};

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: biome_formatter::Format<Context>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_>;
}

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

/// Used to convert this object into an object that can be formatted, taking
/// ownership of `self`.
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

impl<T, Context> IntoFormat<Context> for Option<T>
where
    T: IntoFormat<Context>,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting-specific [Iterator] extensions.
pub(crate) trait FormattedIterExt {
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

pub(crate) type XmlFormatter<'buf> = Formatter<'buf, XmlFormatContext>;

/// Rule for formatting an XML [AstNode].
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = XmlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut XmlFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_node(&self, node: &N, f: &mut XmlFormatter) -> FormatResult<()> {
        self.fmt_fields(node, f)
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut XmlFormatter) -> FormatResult<()>;

    fn is_suppressed(&self, node: &N, f: &XmlFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    fn fmt_leading_comments(&self, node: &N, f: &mut XmlFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    fn fmt_dangling_comments(&self, node: &N, f: &mut XmlFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    fn fmt_trailing_comments(&self, node: &N, f: &mut XmlFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting a bogus node.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = XmlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut XmlFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XmlFormatLanguage {
    options: XmlFormatOptions,
}

impl XmlFormatLanguage {
    pub fn new(options: XmlFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for XmlFormatLanguage {
    type SyntaxLanguage = XmlLanguage;
    type Context = XmlFormatContext;
    type FormatRule = FormatXmlSyntaxNode;

    fn is_range_formatting_node(&self, node: &XmlSyntaxNode) -> bool {
        use xml_syntax::{XmlElement, XmlSelfClosingElement};
        XmlElement::can_cast(node.kind()) || XmlSelfClosingElement::can_cast(node.kind())
    }

    fn options(&self) -> &XmlFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        root: &XmlSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &XmlCommentStyle, source_map.as_ref());
        XmlFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Formats a range within an XML document.
pub fn format_range(
    options: XmlFormatOptions,
    root: &XmlSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, XmlFormatLanguage::new(options))
}

/// Formats a whole XML document.
pub fn format_node(
    options: XmlFormatOptions,
    root: &XmlSyntaxNode,
) -> FormatResult<Formatted<XmlFormatContext>> {
    biome_formatter::format_node(root, XmlFormatLanguage::new(options))
}

/// Formats a single node within a document.
pub fn format_sub_tree(options: XmlFormatOptions, root: &XmlSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, XmlFormatLanguage::new(options))
}
