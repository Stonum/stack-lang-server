use crate::prelude::*;
use biome_formatter::{FormatOptions, format_args, write};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxResult};
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
/// `order_by a, b`) using the same flat/balanced/fill `best_fitting!`
/// approach as [write_bracketed_fill_list] instead of [write_wrapping_clause]'s
/// always-one-per-line-when-broken behavior: simple items still pack
/// multiple-per-line, and only a complex item -- and the item right after
/// it -- force their own line.
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
            let (complex, width) = match &element {
                Ok(node) => (is_complex(node), node_width(node)),
                Err(_) => (true, 0),
            };
            (complex, width, formatted.memoized())
        })
        .collect();

    let line_width = f.options().line_width().get() as usize;
    let balanced_breaks = balanced_fill_breaks(&entries, line_width);

    write!(
        f,
        [best_fitting!(
            format_args![
                keyword,
                space(),
                format_with(|f: &mut PsqlFormatter| {
                    f.join_with(space())
                        .entries(entries.iter().map(|(_, _, formatted)| formatted))
                        .finish()
                }),
            ],
            format_args![
                keyword,
                group(&soft_line_indent_or_space(&format_with(
                    |f: &mut PsqlFormatter| write_balanced_fill_entries(
                        &entries,
                        &balanced_breaks,
                        f
                    )
                )))
                .should_expand(true),
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
/// `PsqlExpressionList` grammar node) using a `best_fitting!` with three
/// variants, mirroring `mlang_formatter`'s real call-argument layout
/// (`mlang_formatter/src/rules/expressions/call_arguments.rs`, the `else`
/// branch building `flat_slice`/`expanded_slice`) rather than the simpler
/// always-one-per-line-when-broken [write_wrapping_separated_list] used
/// elsewhere in this formatter:
///
/// 1. Flat: every item on one line, space-separated, no breaks at all.
/// 2. Balanced (tried only if variant 1 doesn't fit): the same complexity-
///    aware breaks as variant 3, but redistributed so lines are as close to
///    equal width as possible instead of greedily cramming each line to the
///    limit -- see [balanced_fill_breaks] for the algorithm and why it's
///    safe as a non-final `best_fitting!` candidate.
/// 3. Expanded (guaranteed-safe fallback if neither above fits): a real
///    `f.fill()` layout where simple items (see [is_simple_expression])
///    still pack multiple-per-line, but any complex item -- and the item
///    right after it -- always starts on its own line. `.should_expand(true)`
///    on the enclosing group forces the surrounding parens' own leading/
///    trailing soft breaks to actually render as newlines+indent; it
///    doesn't affect the fill's own per-item packing decisions, which the
///    fill algorithm makes independently based on remaining width.
///
/// All three variants must each own the parens (not share one `group(..)`
/// wrapped around a shared list), because `best_fitting!`'s chosen variant
/// is a break boundary that always reports as "fits" to anything outside it
/// -- an enclosing `group(&soft_block_indent(..))` around just the list
/// would therefore never see a reason to expand its own leading/trailing
/// soft breaks, leaving the parens without their newline+indent even when
/// a later variant is what's actually printed.
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
            let (is_complex, width) = match &element {
                Ok(expr) => (!is_simple_expression(expr, 0), node_width(expr)),
                Err(_) => (true, 0),
            };
            (is_complex, width, formatted.memoized())
        })
        .collect();

    let line_width = f.options().line_width().get() as usize;
    let balanced_breaks = balanced_fill_breaks(&entries, line_width);

    write!(
        f,
        [best_fitting!(
            format_args![
                l_paren,
                format_with(|f: &mut PsqlFormatter| {
                    f.join_with(space())
                        .entries(entries.iter().map(|(_, _, formatted)| formatted))
                        .finish()
                }),
                r_paren,
            ],
            format_args![
                l_paren,
                group(&soft_block_indent(&format_with(|f: &mut PsqlFormatter| {
                    write_balanced_fill_entries(&entries, &balanced_breaks, f)
                })))
                .should_expand(true),
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
    entries: &[(bool, usize, T)],
    f: &mut PsqlFormatter,
) -> FormatResult<()>
where
    T: Format<PsqlFormatContext>,
{
    let mut filler = f.fill();
    let mut previous_was_complex = false;

    for (is_complex, _, formatted) in entries {
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

/// Source-text length of `node`, used as a cheap proxy for its printed
/// width. Only ever consulted for items already classified "simple" (see
/// [is_simple_expression]) by the time it matters for balancing, so this
/// stays close to the real printed width in practice (no internal
/// reformatting worth the cost of a second, real format+print pass, unlike
/// e.g. `mlang_formatter`'s `write_with_custom_line_width`).
fn node_width<N: AstNode>(node: &N) -> usize {
    u32::from(node.syntax().text_trimmed_range().len()) as usize
}

/// Packs `entries` left-to-right at a given per-line `budget` (in
/// characters), starting a new line whenever the next item is complex,
/// follows a complex item, or would overflow `budget` -- the same decision
/// [write_fill_expression_entries] makes, but computed ahead of time from
/// estimated widths instead of `f.fill()`'s live, width-aware packing.
/// `breaks[i] == true` means a hard break precedes item `i`.
fn greedy_fill_breaks<T>(entries: &[(bool, usize, T)], budget: usize) -> Vec<bool> {
    let mut breaks = vec![false; entries.len()];
    let mut current_width = 0usize;
    let mut previous_was_complex = false;

    for (i, (is_complex, width, _)) in entries.iter().enumerate() {
        if i == 0 {
            current_width = *width;
        } else if *is_complex || previous_was_complex {
            breaks[i] = true;
            current_width = *width;
        } else if current_width + 2 + width > budget {
            // "+ 2" accounts for the ", " between same-line items.
            breaks[i] = true;
            current_width = *width;
        } else {
            current_width += 2 + width;
        }
        previous_was_complex = *is_complex;
    }

    breaks
}

fn greedy_fill_line_count<T>(entries: &[(bool, usize, T)], budget: usize) -> usize {
    greedy_fill_breaks(entries, budget)
        .into_iter()
        .filter(|&is_break| is_break)
        .count()
        + 1
}

/// Redistributes `entries` across the *same number of lines* that packing
/// them greedily at the full `line_width` would need, but chosen so every
/// line is as close to equally wide as possible -- unlike plain greedy
/// packing, which crams each line to the limit before wrapping, leaving a
/// short, ragged final line.
///
/// This is the standard "minimize the max line width for a fixed line
/// count" trick: find the narrowest per-line budget `Wb` that still packs
/// into the same line count as the full width does (binary search --
/// `greedy_fill_line_count` is monotonically non-increasing in the budget,
/// so this is valid), then pack at `Wb`.
///
/// **Why this can't overflow the *true* available width**: this function
/// has no way to know the real remaining columns at the point it'll be
/// printed (indentation depth from surrounding subqueries/CTEs isn't known
/// during `fmt`, only at print time -- the same reason `f.fill()` itself
/// only ever measures width live, at print time). The caller is expected
/// to offer this as a `best_fitting!` candidate *before* the guaranteed-
/// correct `f.fill()`-based [write_fill_expression_entries] variant: since
/// `best_fitting!` verifies a candidate's first line against the *real*
/// live width before accepting it, and every line this function produces
/// is bounded by the same `Wb <= line_width`, a passing first-line check
/// guarantees the true available width is >= `Wb`, which every other line
/// respects too (same indent level throughout one list) -- so either the
/// whole candidate is safe, or the first-line check fails and `best_fitting!`
/// falls through to the always-correct fallback. Never emitted on its own.
fn balanced_fill_breaks<T>(entries: &[(bool, usize, T)], line_width: usize) -> Vec<bool> {
    let target_lines = greedy_fill_line_count(entries, line_width);
    let max_item_width = entries
        .iter()
        .map(|(_, width, _)| *width)
        .max()
        .unwrap_or(0);

    let mut lo = max_item_width.max(1);
    let mut hi = line_width.max(lo);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if greedy_fill_line_count(entries, mid) <= target_lines {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    greedy_fill_breaks(entries, lo)
}

/// Prints `entries` using precomputed `breaks` (from [balanced_fill_breaks])
/// instead of letting `f.fill()` decide live -- the breaks are already
/// final, so each boundary is either a plain `space()` (same line) or a
/// `hard_line_break()` (new line), no soft breaks or fits-measurement
/// involved.
fn write_balanced_fill_entries<T>(
    entries: &[(bool, usize, T)],
    breaks: &[bool],
    f: &mut PsqlFormatter,
) -> FormatResult<()>
where
    T: Format<PsqlFormatContext>,
{
    for (i, (_, _, formatted)) in entries.iter().enumerate() {
        if i > 0 {
            if breaks[i] {
                write!(f, [hard_line_break()])?;
            } else {
                write!(f, [space()])?;
            }
        }
        write!(f, [formatted])?;
    }
    Ok(())
}
