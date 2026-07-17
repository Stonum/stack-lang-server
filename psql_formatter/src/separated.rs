// Scaffolding stage: no rule file needs separated-list formatting yet
// (stubs use `format_verbatim_node`), so this is temporarily unused.
#![allow(dead_code)]

use super::AsFormat;
use super::prelude::*;
use super::rules::tokens::FormatPsqlSyntaxToken;
use biome_formatter::FormatRefWithRule;
use biome_formatter::separated::{FormatSeparatedElementRule, FormatSeparatedIter};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use psql_syntax::{PsqlLanguage, PsqlSyntaxToken};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct PsqlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = PsqlLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for PsqlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = PsqlLanguage> + AsFormat<PsqlFormatContext> + 'static,
{
    type Context = PsqlFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, PsqlSyntaxToken, FormatPsqlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a PsqlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type PsqlFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<PsqlLanguage, Node>,
    Node,
    PsqlFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = PsqlLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated(&self, separator: &'static str) -> PsqlFormatSeparatedIter<Self::Node> {
        PsqlFormatSeparatedIter::new(
            self.elements(),
            separator,
            PsqlFormatSeparatedElementRule { node: PhantomData },
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = PsqlLanguage> {}
