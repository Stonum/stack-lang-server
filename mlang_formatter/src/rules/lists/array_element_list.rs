use crate::prelude::*;
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, GroupId, write};

use crate::utils::array::write_array_node;

use crate::context::trailing_commas::FormatTrailingCommas;
use crate::utils::member_chain::SimpleArgument;
use crate::utils::{COMPACT_FILL_THRESHOLD, write_compact_fill, write_with_custom_line_width};
use biome_rowan::{AstNode, AstSeparatedList};
use mlang_syntax::{AnyMArrayElement, MArrayElementList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMArrayElementList {
    group_id: Option<GroupId>,
}
impl_format!(MArrayElementList, FormatMArrayElementList);

impl FormatRuleWithOptions<MArrayElementList> for FormatMArrayElementList {
    type Options = Option<GroupId>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.group_id = options;
        self
    }
}

impl FormatRule<MArrayElementList> for FormatMArrayElementList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MArrayElementList, f: &mut MFormatter) -> FormatResult<()> {
        let layout = if can_concisely_print_array_list(node, f.context().comments()) {
            ArrayLayout::Fill
        } else if node.len() >= COMPACT_FILL_THRESHOLD && has_only_expression_elements(node) {
            ArrayLayout::CompactFill
        } else {
            ArrayLayout::OnePerLine
        };

        match layout {
            ArrayLayout::Fill => {
                let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

                let mut filler = f.fill();

                // Using format_separated is valid in this case as can_print_fill does not allow holes
                for (element, formatted) in node.iter().zip(
                    node.format_separated(",")
                        .with_trailing_separator(trailing_separator)
                        .with_group_id(self.group_id),
                ) {
                    filler.entry(
                        &format_once(|f| {
                            let element = element?;
                            if get_lines_before(element.syntax()) > 1 {
                                write!(f, [empty_line()])
                            } else if f.comments().has_leading_own_line_comment(element.syntax()) {
                                write!(f, [hard_line_break()])
                            } else {
                                write!(f, [soft_line_break_or_space()])
                            }
                        }),
                        &formatted,
                    );
                }

                filler.finish()
            }
            ArrayLayout::CompactFill => {
                let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());
                let group_id = self.group_id;
                // Like call arguments, the compact fill layout packs entries
                // against the narrower `pretty_line_width`, not the full line
                // width.
                let custom_width = f.options().pretty_line_width();

                write_with_custom_line_width(
                    f,
                    custom_width,
                    node.syntax(),
                    format_with(|f| {
                        let entries = node
                            .iter()
                            .zip(
                                node.format_separated(",")
                                    .with_trailing_separator(trailing_separator)
                                    .with_group_id(group_id),
                            )
                            .map(|(element, formatted)| {
                                let (is_simple, leading_lines) = match element {
                                    Ok(AnyMArrayElement::AnyMExpression(expr)) => (
                                        SimpleArgument::from(expr.clone()).is_simple(),
                                        get_lines_before(expr.syntax()),
                                    ),
                                    _ => (false, 0),
                                };
                                (is_simple, leading_lines, formatted)
                            })
                            .collect();

                        write_compact_fill(f, entries)
                    }),
                )
            }
            ArrayLayout::OnePerLine => write_array_node(node, f),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ArrayLayout {
    /// Tries to fit as many array elements on a single line as possible.
    ///
    /// ```javascript
    /// [
    ///     1, 2, 3,
    ///     5, 6,
    /// ]
    /// ```
    Fill,

    /// Applied to long lists (at least [COMPACT_FILL_THRESHOLD] elements, no
    /// holes or spreads): packs runs of simple elements together, but forces
    /// any not-simple element onto its own line.
    /// ```javascript
    /// [
    ///     1, 2, a.b(),
    ///     3, 4,
    /// ]
    /// ```
    CompactFill,

    /// Prints every element on a single line if the whole array expression exceeds the line width, or any
    /// of its elements gets printed in *expanded* mode.
    /// ```javascript
    /// [
    ///     a.b(),
    ///     4,
    ///     3,
    /// ]
    /// ```
    OnePerLine,
}

/// Returns `true` if every element of `list` is a plain expression, i.e. no
/// array holes or spread elements. Required for [ArrayLayout::CompactFill]
/// since it relies on `format_separated`'s default trailing-separator
/// handling, which doesn't special-case holes the way [write_array_node] does.
fn has_only_expression_elements(list: &MArrayElementList) -> bool {
    list.iter()
        .all(|element| matches!(element, Ok(AnyMArrayElement::AnyMExpression(_))))
}

/// Returns true if the provided MArrayElementList could
/// be "fill-printed" instead of breaking each element on
/// a different line.
///
/// The underlying logic only allows lists of literal expressions
/// with 10 or less characters, potentially wrapped in a "short"
/// unary expression (+, -, ~ or !)
pub(crate) fn can_concisely_print_array_list(
    list: &MArrayElementList,
    comments: &MComments,
) -> bool {
    use mlang_syntax::AnyMArrayElement::*;
    use mlang_syntax::AnyMExpression::*;
    use mlang_syntax::MUnaryOperator::*;

    if list.is_empty() {
        return false;
    }

    list.elements().all(|item| {
        let syntax = match item.into_node() {
            Ok(AnyMExpression(AnyMLiteralExpression(
                mlang_syntax::AnyMLiteralExpression::MNumberLiteralExpression(literal),
            ))) => literal.into_syntax(),

            Ok(AnyMExpression(MUnaryExpression(expr))) => {
                let signed = matches!(expr.operator(), Ok(Plus | Minus));
                let argument = expr.argument();

                match argument {
                    Ok(AnyMLiteralExpression(
                        mlang_syntax::AnyMLiteralExpression::MNumberLiteralExpression(literal),
                    )) => {
                        if signed && !comments.has_comments(literal.syntax()) {
                            expr.into_syntax()
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }

            _ => {
                return false;
            }
        };

        // Does not have a line comment ending on the same line
        // ```javascript
        // [ a // not this
        //  b];
        //
        // [
        //   // This is fine
        //   thats
        // ]
        // ```
        !comments
            .trailing_comments(&syntax)
            .iter()
            .filter(|comment| comment.kind().is_line())
            .any(|comment| comment.lines_before() == 0)
    })
}
