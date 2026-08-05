// Scaffolding stage: no rule file needs separated-list formatting yet
// (stubs use `format_verbatim_node`), so this is temporarily unused.
#![allow(dead_code)]

use super::AsFormat;
use super::prelude::*;
use super::rules::tokens::FormatSqlSyntaxToken;
use biome_formatter::FormatRefWithRule;
use biome_formatter::separated::{FormatSeparatedElementRule, FormatSeparatedIter};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use sql_syntax::{SqlLanguage, SqlSyntaxToken};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct SqlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = SqlLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for SqlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = SqlLanguage> + AsFormat<SqlFormatContext> + 'static,
{
    type Context = SqlFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, SqlSyntaxToken, FormatSqlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a SqlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type SqlFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<SqlLanguage, Node>,
    Node,
    SqlFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = SqlLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated(&self, separator: &'static str) -> SqlFormatSeparatedIter<Self::Node> {
        SqlFormatSeparatedIter::new(
            self.elements(),
            separator,
            SqlFormatSeparatedElementRule { node: PhantomData },
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = SqlLanguage> {}
