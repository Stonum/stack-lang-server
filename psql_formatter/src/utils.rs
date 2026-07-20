use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_rowan::{AstSeparatedList, SyntaxResult};
use psql_syntax::{
    AnyPsqlExpression, PsqlExpressionList, PsqlFromClause, PsqlGroupByClause, PsqlHavingClause,
    PsqlLanguage, PsqlSelectClause, PsqlSyntaxToken, PsqlWhereClause,
};

/// Prints the `select ... [from ...] [where ...] [group_by ...] [having
/// ...]` core shared by a full `PsqlSelectStatement` and a `PsqlSetOperation`
/// branch, joining present clauses with `soft_line_break_or_space()`. Does
/// *not* create its own `group(..)` -- the caller wraps the call in one, so
/// a simple, short statement collapses onto a single line, while anything
/// containing a clause that itself must hard-break (a JOIN, a wrapped list,
/// an and/or chain of more than two conditions, a subquery) naturally
/// forces the whole thing to expand instead, one clause per line, via the
/// same group-expansion propagation already relied on elsewhere.
pub(crate) fn write_select_body_clauses(
    select_clause: SyntaxResult<PsqlSelectClause>,
    from_clause: Option<PsqlFromClause>,
    where_clause: Option<PsqlWhereClause>,
    group_by_clause: Option<PsqlGroupByClause>,
    having_clause: Option<PsqlHavingClause>,
    f: &mut PsqlFormatter,
) -> FormatResult<()> {
    write!(f, [select_clause.format()])?;
    if let Some(from_clause) = from_clause {
        write!(f, [soft_line_break_or_space(), from_clause.format()])?;
    }
    if let Some(where_clause) = where_clause {
        write!(f, [soft_line_break_or_space(), where_clause.format()])?;
    }
    if let Some(group_by_clause) = group_by_clause {
        write!(f, [soft_line_break_or_space(), group_by_clause.format()])?;
    }
    if let Some(having_clause) = having_clause {
        write!(f, [soft_line_break_or_space(), having_clause.format()])?;
    }
    Ok(())
}

/// Prints a `keyword <wrapping list>` clause (e.g. `select a, b`, `group_by
/// a, b`), wrapping keyword and list together in a `group(..)` so the list
/// indents onto its own lines when it doesn't fit -- *except* when the list
/// has at most one item, in which case it's printed flat with no group at
/// all.
///
/// The single-item exception exists because a `group(..)`  always expands
/// if its content contains a hard line break anywhere inside, no matter how
/// deeply nested (confirmed via `biome_formatter`'s `propagate_expand`) --
/// so a single item whose own formatting must hard-break (e.g. a call
/// expression whose arguments don't fit) would otherwise force the keyword
/// itself onto its own line too, even though "does this comma list of
/// items wrap" isn't a meaningful question with only one item. Same fix as
/// `PsqlFromClause`/`PsqlDeleteUsingClause` apply for the same reason.
pub(crate) fn write_wrapping_clause<L>(
    keyword: SyntaxResult<PsqlSyntaxToken>,
    list: &L,
    f: &mut PsqlFormatter,
) -> FormatResult<()>
where
    L: AstSeparatedList<Language = PsqlLanguage> + AsFormat<PsqlFormatContext>,
{
    if list.len() <= 1 {
        write!(f, [keyword.format(), space(), list.format()])
    } else {
        write!(
            f,
            [group(&format_args![
                keyword.format(),
                soft_line_indent_or_space(&list.format())
            ])]
        )
    }
}

/// Prints a `keyword <list>` clause (`select a, b`, `group_by a, b`,
/// `order_by a, b`) using the same flat/fill `best_fitting!` approach as
/// [write_bracketed_fill_list] instead of [write_wrapping_clause]'s
/// always-one-per-line-when-broken behavior: simple items still pack
/// multiple-per-line via `f.fill()`, and only a complex item -- and the
/// item right after it -- force their own line.
///
/// Unlike [write_bracketed_fill_list] there's no closing bracket to anchor
/// indentation to, so the expanded variant uses `soft_line_indent_or_space`
/// (leading break+indent only, no trailing one) instead of
/// `soft_block_indent`, matching [write_wrapping_clause]'s own choice for
/// the same reason; the single-item flat-no-group exception is preserved
/// too, same rationale as there.
///
/// `is_complex` decides each list item's complexity from its node; callers
/// adapt [is_simple_expression] to whatever expression the list's item
/// wrapper node actually holds (e.g. unwrapping a `PsqlSelectExpression`'s
/// `expr` or a `PsqlOrderByExpression`'s `item`, ignoring the alias/order-
/// direction suffix since that doesn't affect complexity).
pub(crate) fn write_wrapping_fill_clause<L>(
    keyword: SyntaxResult<PsqlSyntaxToken>,
    list: &L,
    is_complex: impl Fn(&L::Node) -> bool,
    f: &mut PsqlFormatter,
) -> FormatResult<()>
where
    L: FormatAstSeparatedListExtension + AsFormat<PsqlFormatContext>,
    L::Node: AsFormat<PsqlFormatContext> + 'static,
{
    if list.len() <= 1 {
        return write!(f, [keyword.format(), space(), list.format()]);
    }

    let keyword = keyword.format().memoized();

    let entries: Vec<_> = list
        .iter()
        .zip(
            list.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit),
        )
        .map(|(element, formatted)| {
            let complex = element.map(|node| is_complex(&node)).unwrap_or(true);
            (complex, formatted.memoized())
        })
        .collect();

    write!(
        f,
        [best_fitting!(
            format_args![
                keyword,
                space(),
                format_with(|f: &mut PsqlFormatter| {
                    f.join_with(space())
                        .entries(entries.iter().map(|(_, formatted)| formatted))
                        .finish()
                }),
            ],
            format_args![
                keyword,
                group(&soft_line_indent_or_space(&format_with(
                    |f: &mut PsqlFormatter| write_fill_expression_entries(&entries, f)
                )))
                .should_expand(true),
            ]
        )]
    )
}

/// Prints a comma-separated list's items joined by `soft_line_break_or_space`.
/// Unlike JS, SQL has no trailing-comma-after-the-last-item syntax at all, so
/// this always uses `TrailingSeparator::Omit` -- a comma appears *between*
/// items (on the previous line, once the group breaks) but never after the
/// last one. This only emits the *items themselves*; the caller is
/// responsible for wrapping the clause keyword and this list together in a
/// `group(..)` (with `soft_line_indent_or_space`/`soft_block_indent` as
/// appropriate for the clause's shape), since the wrap/no-wrap decision has
/// to cover both, not just the list in isolation.
pub(crate) fn write_wrapping_separated_list<L>(list: &L, f: &mut PsqlFormatter) -> FormatResult<()>
where
    L: FormatAstSeparatedListExtension,
    L::Node: AsFormat<PsqlFormatContext> + 'static,
{
    f.join_with(soft_line_break_or_space())
        .entries(
            list.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit),
        )
        .finish()
}

/// Returns `true` for expressions "simple" enough to pack multiple-per-line
/// in a fill layout rather than always taking their own line -- literals,
/// bare names/column references, `*`, a trivial unary (`-x`), or a call/array
/// whose own contents are themselves simple up to a depth of 2. Adapted from
/// `mlang_formatter`'s `SimpleArgument::is_simple`
/// (`mlang_formatter/src/utils/member_chain/simple_argument.rs`), trimmed to
/// the expression kinds SQL actually has (no objects, spreads, member
/// chains, etc.).
pub(crate) fn is_simple_expression(expr: &AnyPsqlExpression, depth: u8) -> bool {
    if depth >= 2 {
        return false;
    }

    match expr {
        AnyPsqlExpression::AnyPsqlLiteralExpression(_)
        | AnyPsqlExpression::PsqlName(_)
        | AnyPsqlExpression::PsqlColReference(_)
        | AnyPsqlExpression::PsqlTableColReference(_)
        | AnyPsqlExpression::PsqlStar(_) => true,
        AnyPsqlExpression::PsqlUnaryExpression(unary) => unary
            .expression()
            .is_ok_and(|inner| is_simple_expression(&inner, depth)),
        AnyPsqlExpression::PsqlCallExpression(call) => {
            let arguments = call.arguments();
            arguments.len() as u8 + depth <= 2
                && arguments
                    .iter()
                    .all(|arg| arg.is_ok_and(|arg| is_simple_expression(&arg, depth + 1)))
        }
        AnyPsqlExpression::PsqlArrayExpression(array) => array
            .items()
            .iter()
            .all(|item| item.is_ok_and(|item| is_simple_expression(&item, depth + 1))),
        _ => false,
    }
}

/// Prints `l_paren <list> r_paren` (a call's arguments, `INSERT ...
/// VALUES (...)`, or an array literal's `[...]` -- all four reuse this one
/// `PsqlExpressionList` grammar node) using a `best_fitting!` with two
/// variants, mirroring `mlang_formatter`'s real call-argument layout
/// (`mlang_formatter/src/rules/expressions/call_arguments.rs`, the `else`
/// branch building `flat_slice`/`expanded_slice`) rather than the simpler
/// always-one-per-line-when-broken [write_wrapping_separated_list] used
/// elsewhere in this formatter:
///
/// 1. Flat: every item on one line, space-separated, no breaks at all.
/// 2. Expanded (used only if variant 1 doesn't fit): a fill layout where
///    simple items (see [is_simple_expression]) still pack multiple-per-line,
///    but any complex item -- and the item right after it -- always starts
///    on its own line. `.should_expand(true)` on the enclosing group forces
///    the surrounding parens' own leading/trailing soft breaks to actually
///    render as newlines+indent; it doesn't affect the fill's own per-item
///    packing decisions, which the fill algorithm makes independently based
///    on remaining width.
///
/// The two variants must each own the parens (not share one `group(..)`
/// wrapped around a shared list), because `best_fitting!`'s chosen variant
/// is a break boundary that always reports as "fits" to anything outside it
/// -- an enclosing `group(&soft_block_indent(..))` around just the list
/// would therefore never see a reason to expand its own leading/trailing
/// soft breaks, leaving the parens without their newline+indent even when
/// the expanded variant is what's actually printed.
pub(crate) fn write_bracketed_fill_list(
    l_paren: SyntaxResult<PsqlSyntaxToken>,
    list: &PsqlExpressionList,
    r_paren: SyntaxResult<PsqlSyntaxToken>,
    f: &mut PsqlFormatter,
) -> FormatResult<()> {
    if list.len() == 0 {
        return write!(f, [l_paren.format(), r_paren.format()]);
    }

    // Both the parens and each item are referenced from *both*
    // `best_fitting!` variants below (only one of which ever actually gets
    // printed), but the token tracker only allows a given token to be
    // printed once per pass. Memoizing here caches each formatted
    // representation so both variants can reuse it without re-triggering
    // the tracker -- same fix `call_arguments.rs` applies to its own parens
    // and arguments for the same reason (there, via `f.intern`/`will_break`
    // transitioning each argument to its cached `Inspected` form before it's
    // referenced from more than one variant).
    let l_paren = l_paren.format().memoized();
    let r_paren = r_paren.format().memoized();

    let entries: Vec<_> = list
        .iter()
        .zip(
            list.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit),
        )
        .map(|(element, formatted)| {
            let is_complex = !element.is_ok_and(|expr| is_simple_expression(&expr, 0));
            (is_complex, formatted.memoized())
        })
        .collect();

    write!(
        f,
        [best_fitting!(
            format_args![
                l_paren,
                format_with(|f: &mut PsqlFormatter| {
                    f.join_with(space())
                        .entries(entries.iter().map(|(_, formatted)| formatted))
                        .finish()
                }),
                r_paren,
            ],
            format_args![
                l_paren,
                group(&soft_block_indent(&format_with(|f: &mut PsqlFormatter| {
                    write_fill_expression_entries(&entries, f)
                })))
                .should_expand(true),
                r_paren,
            ]
        )]
    )
}

fn write_fill_expression_entries<T>(
    entries: &[(bool, T)],
    f: &mut PsqlFormatter,
) -> FormatResult<()>
where
    T: Format<PsqlFormatContext>,
{
    let mut filler = f.fill();
    let mut previous_was_complex = false;

    for (is_complex, formatted) in entries {
        let after_complex = previous_was_complex;

        filler.entry(
            &format_once(|f| {
                if *is_complex || after_complex {
                    write!(f, [hard_line_break()])
                } else {
                    write!(f, [soft_line_break_or_space()])
                }
            }),
            formatted,
        );

        previous_was_complex = *is_complex;
    }

    filler.finish()
}
