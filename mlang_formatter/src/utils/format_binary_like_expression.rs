//! This module implements the formatting of binary like nodes.
//! Binary like nodes are nodes with `left` and `right` expressions. They include:
//! - [MBinaryExpression]
//! - [MLogicalExpression]
//! - [MInExpression]
//! - [MInstanceofExpression]
//!
//! The challenge of formatting binary like expressions is that we want to format binary expression
//! chains, when possible, together but they are represented as a deep structured tree in the CST.
//!
//! For example,
//!
//! ```JavaScript
//! some && thing && elsewhere || happy
//! ```
//!
//! Is parsed as
//!
//! ```block
//! MLogicalExpression {
//!     left: MLogicalExpression {
//!         left: MLogicalExpression {
//!             left: "some"
//!             operator: "&&",
//!             right: "thing"
//!         }
//!         operator: "&&"
//!         right: "elsewhere"
//!     }
//!     operator: "||"
//!     right: "happy"
//! }
//! ```
//!
//! The goal is to format all the left and right sides together that don't require parentheses (mainly comes down to whether the parent and its left side's operator have the same precedence).
//!
//! This is achieved by traversing down the left side of a binary expression until it reaches the first expression that can't be flattened.
//! For `some && thing && elsewhere || happy`, the implementation checks if the first left-side `some && thing && elsewhere` can be grouped.
//! This isn't the case because the left side operator `&&` differs from the parent's `||` operator.
//!
//! That means, we found the end of the first `group` and the left-side of the group is `some && thing && elsewhere`.
//! The algorithm traverses upwards and adds all right-sides of the parent binary like expressions to the group until it reaches the root.
//! In the example, this only is the `|| happy`.
//!
//! Thus, the first group is: `[Left(some && thing && elsewhere), Right(|| happy)]`. The formatting formats the left side
//! as is (the call will recurse into the [AnyMBinaryLikeExpression] formatting again) but formats the operator with the right side.
//!
//! Now, let's see how the implementation groups the `some && thing && elsewhere`. It first traverses to the left most binary like expression,
//! which is `some && thing`. It then adds this as a `Left` side to the group. From here, the algorithm traverses upwards and adds all right sides
//! of the binary expression. These are: `&& thing` and `&& elsewhere`.
//! The complete group is: `[Left(some), Right(&& thing), Right(&& elsewhere)]`.
//!
//! Each side in the group gets formatted in order, starting with the left, then formatting the operator
//! and right side of each Right side.

use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use biome_formatter::{Buffer, CstFormatContext, FormatOptions, format_args, write};
use mlang_syntax::binary_like_expression::{
    AnyMBinaryLikeExpression, AnyMBinaryLikeLeftExpression,
};
use mlang_syntax::{
    AnyMExpression, AnyMLiteralExpression, MBinaryOperator, MSyntaxKind, MSyntaxNode, MSyntaxToken,
    MUnaryExpression,
};

use crate::rules::expressions::static_member_expression::AnyMStaticMemberLike;
use biome_rowan::{AstNode, SyntaxResult};
use std::fmt::Debug;
use std::iter::FusedIterator;

impl Format<MFormatContext> for AnyMBinaryLikeExpression {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        if try_format_flat_multiline_concatenation(self, f)? {
            return Ok(());
        }

        let parent = self.syntax().parent();

        let is_inside_condition = self.is_inside_condition(parent.as_ref());
        let parts = split_into_left_and_right_sides(self, is_inside_condition)?;

        // Don't indent inside of conditions because conditions add their own indent and grouping.
        if is_inside_condition {
            return write!(f, [&format_once(|f| { f.join().entries(parts).finish() })]);
        }

        if let Some(parent) = parent.as_ref() {
            // Add a group with a soft block indent in cases where it is necessary to parenthesize the binary expression.
            // For example, `(a+b)(call)`, `!(a + b)`, `(a + b).test`.
            let is_callee = matches!(
                parent.kind(),
                MSyntaxKind::M_CALL_EXPRESSION | MSyntaxKind::M_NEW_EXPRESSION
            );
            if is_callee
                || MUnaryExpression::can_cast(parent.kind())
                || AnyMStaticMemberLike::can_cast(parent.kind())
            {
                return write!(
                    f,
                    [group(&soft_block_indent(&format_once(|f| {
                        f.join().entries(parts).finish()
                    })))]
                );
            }
        }

        let inline_logical_expression = self.should_inline_logical_expression();
        let should_indent_if_inlines = should_indent_if_parent_inlines(parent.as_ref());
        let should_not_indent = self.should_not_indent_if_parent_indents(parent);

        let flattened = parts.len() > 2;

        if should_not_indent
            || (inline_logical_expression && !flattened)
            || (!inline_logical_expression && should_indent_if_inlines)
        {
            return write!(
                f,
                [group(&format_once(|f| {
                    f.join().entries(parts).finish()
                }))]
            );
        }

        if let Some(first) = parts.first() {
            let tail_parts = &parts[1..];

            let group_id = f.group_id("logicalChain");

            let format_parts = format_with(|f| {
                write!(
                    f,
                    [group(&format_args![
                        first,
                        indent(&format_once(|f| {
                            f.join().entries(tail_parts.iter()).finish()
                        }))
                    ])
                    .with_group_id(Some(group_id))]
                )
            });

            write!(f, [format_parts])
        } else {
            // Empty, should never ever happen but let's gracefully recover.
            Ok(())
        }
    }
}

/// Creates a [BinaryLeftOrRightSide::Left] for the first left hand side that:
/// * isn't a [MBinaryLikeExpression]
/// * is a [MBinaryLikeExpression] but it should be formatted as its own group (see [AnyMBinaryLikeExpression::can_flatten]).
///
/// It then traverses upwards from the left most node and creates [BinaryLikeLeftOrRightSide::Right]s for
/// every [MBinaryLikeExpression] until it reaches the root again.
fn split_into_left_and_right_sides(
    root: &AnyMBinaryLikeExpression,
    inside_condition: bool,
) -> SyntaxResult<Vec<BinaryLeftOrRightSide>> {
    // Stores the left and right parts of the binary expression in sequence (rather than nested as they
    // appear in the tree).
    let mut items = Vec::new();

    let mut expressions = BinaryLikePreorder::new(root.clone());

    while let Some(event) = expressions.next() {
        match event {
            VisitEvent::Enter(binary) => {
                if !binary.can_flatten()? {
                    // Stop at this expression. This is either not a binary expression OR it has
                    // different precedence and needs to be grouped separately.
                    // Calling skip_subtree prevents the exit event being triggered for this event.
                    expressions.skip_subtree();

                    items.push(BinaryLeftOrRightSide::Left { parent: binary });
                }
            }
            VisitEvent::Exit(expression) => items.push(BinaryLeftOrRightSide::Right {
                print_parent_comments: expression.syntax() != root.syntax(),
                parent: expression,
                inside_condition,
            }),
        }
    }

    Ok(items)
}

/// Width, in columns, contributed by joining two operands together
/// (`" + "`).
const OPERATOR_WIDTH: usize = 3;

/// How much horizontal space an operand of a `+` chain takes up.
enum OperandWidth {
    /// Prints on a single line, this many columns wide.
    Single(usize),
    /// A multi-line string literal -- one entry per printed line, in order.
    /// Always has at least 2 entries.
    Multiline(Vec<usize>),
}

/// A `+`-only chain, already validated as eligible for
/// [try_format_flat_multiline_concatenation]'s flat rendering.
pub(crate) struct FlatMultilineConcatenation {
    operands: Vec<AnyMExpression>,
    operators: Vec<MSyntaxToken>,
    /// Indices into `operands` of the multi-line string literals, in order
    /// (there can be more than one).
    multiline_indices: Vec<usize>,
}

/// Tries to format a `+`-only chain of "simple" operands (identifiers,
/// numbers, booleans, string literals) that contains one or more multi-line
/// string literals, keeping every operand on the same visual line instead of
/// letting `biome_formatter`'s `Document::propagate_expand` force each
/// operand of the chain onto its own line just because *one* of them
/// happens to embed a raw newline -- `propagate_expand` unconditionally
/// expands every group that (transitively) contains a hard break or a text
/// run with `\n`, with no regard for whether the content before that break
/// actually needs to wrap. Returns `Ok(true)` if it wrote a flat rendering
/// (caller must not format anything else for this node), `Ok(false)` if the
/// chain doesn't qualify or wouldn't fit, in which case the caller falls
/// back to the regular group-based formatting further down.
fn try_format_flat_multiline_concatenation(
    root: &AnyMBinaryLikeExpression,
    f: &mut Formatter<MFormatContext>,
) -> FormatResult<bool> {
    let Some(plan) = plan_flat_multiline_concatenation(root, f)? else {
        return Ok(false);
    };

    // Every operator in this chain is a plain `+` (checked above); mark the
    // original tokens consumed and reconstruct the visible ` + ` separators
    // from scratch, the same way `FormatConcatenatedQuery` does for
    // embedded-SQL concatenation (`concatenation.rs`).
    for operator in &plan.operators {
        write!(f, [format_removed(operator)])?;
    }

    let first_multiline = plan.multiline_indices[0];
    let prefix = &plan.operands[..first_multiline];

    if let Some((first, tail)) = prefix.split_first() {
        // A plain group -- its own content never embeds a raw newline
        // (every prefix operand is one of the "simple", breakless kinds
        // `plan_flat_multiline_concatenation` already checked), so
        // `Document::propagate_expand` never force-expands it on its own.
        // `fits()` keeps scanning past a group's own end into whatever
        // follows in the queue -- here, the first multi-line literal's own
        // first line, written right after this group -- so the group's
        // flat/expanded decision reflects this expression's actual ambient
        // column at print time, unlike a plain arithmetic width check
        // computed while building the IR, before printing (and so before
        // there's any real column to read) has even started.
        write!(
            f,
            [group(&format_with(|f| {
                write!(f, [first.format()])?;
                write!(
                    f,
                    [indent(&format_with(|f| {
                        for operand in tail {
                            write!(
                                f,
                                [soft_line_break_or_space(), text("+ "), operand.format()]
                            )?;
                        }
                        Ok(())
                    }))]
                )
            }))]
        )?;
        write!(f, [text(" + ")])?;
    }

    // From the first multi-line operand onward, everything is static and
    // flat, verified to fit by `plan_flat_multiline_concatenation`: every
    // operand from here on either starts right after a raw embedded
    // newline (which the printer never re-indents, so it starts at a known
    // column 0) or is single-line content immediately following one on the
    // same physical line, so its own starting column is knowable too --
    // unlike the dynamic prefix segment above, none of this needs a
    // `group()`/`fits()` check.
    for (index, operand) in plan.operands[first_multiline..].iter().enumerate() {
        let index = first_multiline + index;
        if index > first_multiline {
            write!(f, [text(" + ")])?;
        }

        if plan.multiline_indices.contains(&index) {
            write_flat_multiline_operand(operand, f)?;
        } else {
            write!(f, [operand.format()])?;
        }
    }

    Ok(true)
}

/// Writes a multi-line string literal operand as two separate text runs
/// split right at its first embedded newline, rather than `operand.format()`'s
/// normal single run, so a preceding `group()`'s `fits()` check (see the
/// prefix handling in [try_format_flat_multiline_concatenation]) can
/// validate this first line's width the *normal* way. A single text run
/// whose content straddles the `\n` trips a quirk in the vendored
/// `biome_formatter`'s `fits_text`: it returns "fits" the instant it
/// reaches an embedded `\n`, without ever comparing the width accumulated
/// so far against the line-width budget -- so if this line's width were
/// only validated as part of one run together with the literal, an
/// overlong line would wrongly be judged as fitting. Splitting at the
/// newline sidesteps that: the first run (with no `\n` of its own) goes
/// through the normal, correct check.
fn write_flat_multiline_operand(
    operand: &AnyMExpression,
    f: &mut Formatter<MFormatContext>,
) -> FormatResult<()> {
    let token = string_literal_token(operand).expect(
        "every index in `multiline_indices` is one `classify_operand_width` identified as \
         OperandWidth::Multiline, which is always a string literal",
    );
    f.context()
        .comments()
        .mark_suppression_checked(operand.syntax());
    write!(f, [format_removed(&token)])?;
    let (first_line, rest) = split_multiline_literal_text(&token).expect(
        "classify_operand_width already confirmed this literal's cleaned text contains '\\n'",
    );
    let start = token.text_trimmed_range().start();
    write!(f, [dynamic_text(&first_line, start)])?;
    write!(f, [dynamic_text(&rest, start)])
}

/// The `value_token` of `expression` if it's a string literal
/// (`MStringLiteralExpression` or `MLongStringLiteralExpression`; mlang's
/// `` ` ``/`"` delimiters are interchangeable, see `string_utils.rs`).
fn string_literal_token(expression: &AnyMExpression) -> Option<MSyntaxToken> {
    match expression {
        AnyMExpression::AnyMLiteralExpression(AnyMLiteralExpression::MStringLiteralExpression(
            string,
        )) => string.value_token().ok(),
        AnyMExpression::AnyMLiteralExpression(
            AnyMLiteralExpression::MLongStringLiteralExpression(string),
        ) => string.value_token().ok(),
        _ => None,
    }
}

/// Splits a multi-line string literal's cleaned (quoted, escaped) text at
/// its first embedded newline, into `(up to but excluding "\n", "\n" and
/// everything after)`. Returns `None` if the text has no embedded newline.
fn split_multiline_literal_text(token: &MSyntaxToken) -> Option<(String, String)> {
    let formatter = FormatLiteralStringToken::new(token, StringLiteralParentKind::Expression);
    let cleaned = formatter.clean_text();
    let text = cleaned.text();
    let newline_at = text.find('\n')?;

    Some((
        text[..newline_at].to_string(),
        text[newline_at..].to_string(),
    ))
}

/// Whether `expression` is a `+`-chain that [try_format_flat_multiline_concatenation]
/// would render flat -- used by `assignment_like.rs` to keep the `=` glued
/// to the right-hand side for the same reason it already does for a bare
/// multi-line string literal (see `string_prints_multiline`): the chain
/// carries its own internal layout (flat, in this case) and stacking the
/// assignment's own `BreakAfterOperator` soft-break/indent on top of that
/// would just push `=` onto its own line for no reason.
pub(crate) fn is_flat_multiline_concatenation_candidate(
    expression: &AnyMExpression,
    f: &Formatter<MFormatContext>,
) -> SyntaxResult<bool> {
    let Some(binary_like) = AnyMBinaryLikeExpression::cast(expression.syntax().clone()) else {
        return Ok(false);
    };

    Ok(plan_flat_multiline_concatenation(&binary_like, f)?.is_some())
}

/// Deliberately narrow in scope: every operand must be one of the "simple"
/// kinds (identifiers, numbers, booleans, string literals) so that its
/// printed width can be computed without actually formatting it -- a
/// nested call, array, etc. bails out instead of risking a wrong width.
fn plan_flat_multiline_concatenation(
    root: &AnyMBinaryLikeExpression,
    f: &Formatter<MFormatContext>,
) -> SyntaxResult<Option<FlatMultilineConcatenation>> {
    let AnyMBinaryLikeExpression::MBinaryExpression(binary) = root else {
        return Ok(None);
    };
    if binary.operator()? != MBinaryOperator::Plus {
        return Ok(None);
    }

    let parts = split_into_left_and_right_sides(root, false)?;

    let mut operands = Vec::with_capacity(parts.len());
    let mut operators = Vec::with_capacity(parts.len().saturating_sub(1));

    for part in &parts {
        match part {
            BinaryLeftOrRightSide::Left { parent } => {
                let Some(expression) = parent.left()?.into_expression() else {
                    return Ok(None);
                };
                operands.push(expression);
            }
            BinaryLeftOrRightSide::Right { parent, .. } => {
                // `-` shares `+`'s operator precedence, so a chain like
                // `a + b - c` gets flattened together by
                // `split_into_left_and_right_sides` -- bail rather than
                // treat `-` as if it were concatenation.
                let AnyMBinaryLikeExpression::MBinaryExpression(parent_binary) = parent else {
                    return Ok(None);
                };
                if parent_binary.operator()? != MBinaryOperator::Plus {
                    return Ok(None);
                }
                // This function never calls `parent.format()` -- it only
                // ever formats the terminal operands directly -- so, like
                // `BinaryLeftOrRightSide::Right`'s own formatting further
                // down, it must mark every intermediate binary-expression
                // node it bypasses as checked for a suppression comment by
                // hand, or a debug build panics.
                f.context()
                    .comments()
                    .mark_suppression_checked(parent.syntax());
                operators.push(parent.operator_token()?);
                operands.push(parent.right()?);
            }
        }
    }

    let comments = f.context().comments();
    if operands
        .iter()
        .any(|operand| comments.has_comments(operand.syntax()))
        || operators
            .iter()
            .filter_map(|token| token.parent())
            .any(|node| comments.has_comments(&node))
    {
        return Ok(None);
    }

    let mut multiline_indices = Vec::new();
    let mut widths = Vec::with_capacity(operands.len());
    for (index, operand) in operands.iter().enumerate() {
        let Some(width) = classify_operand_width(operand)? else {
            return Ok(None);
        };
        if matches!(width, OperandWidth::Multiline(_)) {
            multiline_indices.push(index);
        }
        widths.push(width);
    }

    if multiline_indices.is_empty() {
        // No multi-line literal in the chain -- the regular group-based
        // formatting already handles this correctly.
        return Ok(None);
    }

    let budget = f.options().line_width().get() as usize;

    // Checks the width of every physical line from the first multi-line
    // operand onward (the content *before* it is handled dynamically at
    // print time instead, by the `group()` in
    // `try_format_flat_multiline_concatenation`, since the column it
    // starts at isn't knowable this early). Each multi-line literal's own
    // embedded newline is never re-indented by the printer, so every
    // physical line boundary from here on starts at a known column: a
    // literal's *first* line continues whatever line was already open
    // (single operands since the previous literal, or this whole
    // expression's ambient column for the very first one), and its *last*
    // line starts a fresh one, which single operands after it extend until
    // the next multi-line literal -- or the end of the chain -- closes it.
    let mut running_width: Option<usize> = None;
    for width in &widths {
        match width {
            OperandWidth::Single(width) => {
                if let Some(running_width) = running_width.as_mut() {
                    *running_width += OPERATOR_WIDTH + width;
                }
            }
            OperandWidth::Multiline(lines) => {
                let first_line_width = *lines.first().unwrap();
                if let Some(running_width) = running_width
                    && running_width + OPERATOR_WIDTH + first_line_width > budget
                {
                    return Ok(None);
                }
                running_width = Some(*lines.last().unwrap());
            }
        }
    }
    if running_width.is_some_and(|running_width| running_width > budget) {
        return Ok(None);
    }

    Ok(Some(FlatMultilineConcatenation {
        operands,
        operators,
        multiline_indices,
    }))
}

fn classify_operand_width(expression: &AnyMExpression) -> SyntaxResult<Option<OperandWidth>> {
    let width = match expression {
        AnyMExpression::AnyMLiteralExpression(literal) => match literal {
            AnyMLiteralExpression::MStringLiteralExpression(string) => {
                classify_string_literal(&string.value_token()?)
            }
            AnyMLiteralExpression::MLongStringLiteralExpression(string) => {
                classify_string_literal(&string.value_token()?)
            }
            AnyMLiteralExpression::MNumberLiteralExpression(number) => Some(OperandWidth::Single(
                number.value_token()?.text_trimmed().chars().count(),
            )),
            AnyMLiteralExpression::MBooleanLiteralExpression(boolean) => Some(
                OperandWidth::Single(boolean.value_token()?.text_trimmed().chars().count()),
            ),
            _ => None,
        },
        AnyMExpression::MIdentifierExpression(identifier) => Some(OperandWidth::Single(
            identifier
                .name()?
                .value_token()?
                .text_trimmed()
                .chars()
                .count(),
        )),
        _ => None,
    };

    Ok(width)
}

fn classify_string_literal(token: &MSyntaxToken) -> Option<OperandWidth> {
    let formatter = FormatLiteralStringToken::new(token, StringLiteralParentKind::Expression);
    let cleaned = formatter.clean_text();
    let text = cleaned.text();

    // `chars().count()` (not `.len()`, a byte count) so multi-byte UTF-8
    // scripts like Cyrillic don't get counted as roughly twice their actual
    // printed column width.
    if text.contains('\n') {
        Some(OperandWidth::Multiline(
            text.split('\n').map(|line| line.chars().count()).collect(),
        ))
    } else {
        Some(OperandWidth::Single(text.chars().count()))
    }
}

/// There are cases where the parent decides to inline the element; in
/// these cases the decide to actually break on a new line and indent it.
///
/// This function checks what the parents adheres to this behaviour
fn should_indent_if_parent_inlines(parent: Option<&MSyntaxNode>) -> bool {
    parent.is_some_and(|parent| match parent.kind() {
        MSyntaxKind::M_ASSIGNMENT_EXPRESSION | MSyntaxKind::M_PROPERTY_OBJECT_MEMBER => true,

        MSyntaxKind::M_INITIALIZER_CLAUSE => parent.parent().is_some_and(|grand_parent| {
            matches!(
                grand_parent.kind(),
                MSyntaxKind::M_VARIABLE_DECLARATOR | MSyntaxKind::M_PROPERTY_CLASS_MEMBER
            )
        }),
        _ => false,
    })
}

/// Represents the right or left hand side of a binary expression.
#[derive(Debug, Clone)]
enum BinaryLeftOrRightSide {
    /// A terminal left hand side of a binary expression.
    ///
    /// Formats the left hand side only.
    Left { parent: AnyMBinaryLikeExpression },

    /// The right hand side of a binary expression.
    /// Formats the operand together with the right hand side.
    Right {
        parent: AnyMBinaryLikeExpression,
        /// Is the parent the condition of a `if` / `while` / `do-while` / `for` statement?
        inside_condition: bool,

        /// Indicates if the comments of the parent should be printed or not.
        /// Must be true if `parent` isn't the root `MAnyBinaryLike` for which `format` is called.
        print_parent_comments: bool,
    },
}

impl Format<MFormatContext> for BinaryLeftOrRightSide {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self {
            Self::Left { parent } => {
                write!(f, [group(&parent.left())])
            }
            Self::Right {
                parent: binary_like_expression,
                inside_condition: inside_parenthesis,
                print_parent_comments,
            } => {
                // It's only possible to suppress the formatting of the whole binary expression formatting OR
                // the formatting of the right hand side value but not of a nested binary expression.
                // This aligns with Prettier's behaviour.
                f.context()
                    .comments()
                    .mark_suppression_checked(binary_like_expression.syntax());

                let right = binary_like_expression.right()?;
                let operator_token = binary_like_expression.operator_token()?;

                let operator_and_right_expression = format_with(|f| {
                    let should_inline = binary_like_expression.should_inline_logical_expression();

                    if should_inline {
                        write!(f, [space()])?;
                    } else {
                        write!(f, [soft_line_break_or_space()])?;
                    }

                    write!(f, [operator_token.format(), space(), right.format()])?;

                    Ok(())
                });

                let syntax = binary_like_expression.syntax();
                let parent = syntax.parent();

                // Doesn't match prettier that only distinguishes between logical and binary
                let parent_has_same_kind = parent.as_ref().is_some_and(|parent| {
                    is_same_binary_expression_kind(binary_like_expression, parent)
                });

                let left_has_same_kind = binary_like_expression
                    .left()?
                    .into_expression()
                    .is_some_and(|left| {
                        is_same_binary_expression_kind(binary_like_expression, left.syntax())
                    });
                let right_has_same_kind =
                    is_same_binary_expression_kind(binary_like_expression, right.syntax());

                let should_break = f
                    .context()
                    .comments()
                    .trailing_comments(binary_like_expression.left()?.syntax())
                    .iter()
                    .any(|comment| comment.kind().is_line());

                let should_group = !(parent_has_same_kind
                    || left_has_same_kind
                    || right_has_same_kind
                    || (*inside_parenthesis
                        && matches!(
                            binary_like_expression,
                            AnyMBinaryLikeExpression::MLogicalExpression(_)
                        )));

                if *print_parent_comments {
                    write!(
                        f,
                        [format_leading_comments(binary_like_expression.syntax())]
                    )?;
                }

                if should_group {
                    write!(
                        f,
                        [group(&operator_and_right_expression).should_expand(should_break)]
                    )?;
                } else {
                    write!(f, [operator_and_right_expression])?;
                }

                if *print_parent_comments {
                    write!(
                        f,
                        [format_trailing_comments(binary_like_expression.syntax())]
                    )?;
                }

                Ok(())
            }
        }
    }
}

impl Format<MFormatContext> for AnyMBinaryLikeLeftExpression {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self {
            Self::AnyMExpression(expression) => {
                write![f, [expression.format()]]
            }
        }
    }
}

fn is_same_binary_expression_kind(binary: &AnyMBinaryLikeExpression, other: &MSyntaxNode) -> bool {
    match binary {
        AnyMBinaryLikeExpression::MLogicalExpression(_) => {
            matches!(other.kind(), MSyntaxKind::M_LOGICAL_EXPRESSION)
        }
        AnyMBinaryLikeExpression::MBinaryExpression(_)
        | AnyMBinaryLikeExpression::MInstanceofExpression(_)
        | AnyMBinaryLikeExpression::MInExpression(_) => {
            matches!(
                other.kind(),
                MSyntaxKind::M_BINARY_EXPRESSION
                    | MSyntaxKind::M_IN_EXPRESSION
                    | MSyntaxKind::M_INSTANCEOF_EXPRESSION
            )
        }
    }
}

/// The [BinaryLikePreorder] visits every node twice. First on the way down to find the left most binary
/// like expression, then on the way back up. This enum encodes the information whatever the
/// iterator is on its way down (`Enter`) or traversing upwards (`Exit`).
#[derive(Debug, Eq, PartialEq, Clone)]
enum VisitEvent {
    Enter(AnyMBinaryLikeExpression),
    Exit(AnyMBinaryLikeExpression),
}

/// Iterator that visits [AnyMBinaryLikeExpression]s in pre-order.
/// This is similar to [MSyntaxNode::descendants] but it only traverses into [AnyMBinaryLikeExpression] and their left side
/// (the right side is never visited).
///
/// # Examples
///
/// ```JavaScript
/// a && b && c && d
/// ```
/// This produces a tree with the following shape:
///
/// ```txt
///         &&
///        / \
///       /   \
///      &&   d && e
///     / \
///    /   \
///   &&    c
///  / \
/// a   b
/// ```
///
/// The iterator emits the following events:
///
/// * Enter(`a && b && c && d && e`)
/// * Enter(`a && b && c`)
/// * Enter(`a && b`)
/// * Exit(`a && b`)
/// * Exit(`a && b && c`)
/// * Exit(`a && b && c && d && e`)
///
/// Notice how the iterator doesn't yield events for the terminal identifiers `a`, `b`, `c`, `d`, and `e`,
/// nor for the right hand side expression `d && e`. This is because the visitor only traverses into
/// [AnyMBinaryLikeExpression]s and of those, only along the left side.
struct BinaryLikePreorder {
    /// The next node to visit or [None] if the iterator passed the start node (is at its end).
    next: Option<VisitEvent>,

    /// The start node. Necessary to know when to stop iterating.
    start: MSyntaxNode,

    skip_subtree: bool,
}

impl BinaryLikePreorder {
    fn new(start: AnyMBinaryLikeExpression) -> Self {
        Self {
            start: start.syntax().clone(),
            next: Some(VisitEvent::Enter(start)),
            skip_subtree: false,
        }
    }

    fn skip_subtree(&mut self) {
        self.next = self.next.take().and_then(|next| match next {
            VisitEvent::Enter(binary) => {
                if binary.syntax() == &self.start {
                    None
                } else {
                    // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                    // if it is a valid binary like expression and it is guaranteed to have a parent.
                    let expression = binary
                        .syntax()
                        .parent()
                        .and_then(AnyMBinaryLikeExpression::cast)
                        .unwrap();

                    Some(VisitEvent::Exit(expression))
                }
            }
            VisitEvent::Exit(node) => Some(VisitEvent::Exit(node)),
        });
        self.skip_subtree = false;
    }
}

impl Iterator for BinaryLikePreorder {
    type Item = VisitEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_subtree {
            self.skip_subtree();
        }

        let next = self.next.take()?;
        match &next {
            VisitEvent::Enter(binary) => {
                let next = binary
                    .left()
                    .ok()
                    .and_then(|left| left.into_expression())
                    .and_then(|expression| {
                        AnyMBinaryLikeExpression::cast(expression.into_syntax())
                    });

                if let Some(binary) = next {
                    self.next = Some(VisitEvent::Enter(binary));
                } else {
                    // If left is missing or it isn't a binary like expression, then format it as part of the parent binary like expression
                    self.next = Some(VisitEvent::Exit(binary.clone()));
                }
            }
            VisitEvent::Exit(node) => {
                if node.syntax() != &self.start {
                    self.next = node.syntax().parent().map(|parent| {
                        // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                        // if it is a valid binary like expression.
                        let expression = AnyMBinaryLikeExpression::cast(parent).unwrap();
                        VisitEvent::Exit(expression)
                    });
                }
            }
        };

        Some(next)
    }
}

impl FusedIterator for BinaryLikePreorder {}
