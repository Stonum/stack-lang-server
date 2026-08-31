use crate::prelude::*;
use biome_formatter::format_element::document::Document;
use biome_formatter::{FormatOptions, Formatted, VecBuffer, format_args, write};
use biome_rowan::{AstSeparatedList, SyntaxResult};
use sql_syntax::{
    AnySqlExpression, AnySqlFromExpression, SqlFromClause, SqlFromItem, SqlGroupByClause,
    SqlHavingClause, SqlSelectClause, SqlSyntaxToken, SqlWhereClause,
};

/// Prints the `select ... [from ...] [where ...] [group_by ...] [having
/// ...]` core shared by a full `SqlSelectStatement` and a `SqlSetOperation`
/// branch, joining present clauses with `soft_line_break_or_space()`. Does
/// *not* create its own `group(..)` -- the caller wraps the call in one, so
/// a simple, short statement collapses onto a single line, while anything
/// containing a clause that itself must hard-break (a JOIN, a wrapped list,
/// an and/or chain of more than two conditions, a subquery) naturally
/// forces the whole thing to expand instead, one clause per line, via the
/// same group-expansion propagation already relied on elsewhere.
pub(crate) fn write_select_body_clauses(
    select_clause: SyntaxResult<SqlSelectClause>,
    from_clause: Option<SqlFromClause>,
    where_clause: Option<SqlWhereClause>,
    group_by_clause: Option<SqlGroupByClause>,
    having_clause: Option<SqlHavingClause>,
    f: &mut SqlFormatter,
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

/// Prints a `keyword <list>` clause (`select a, b`, `group_by a, b`,
/// `order_by a, b`) using the same flat/balanced/fill `best_fitting!`
/// approach as [write_bracketed_fill_list]: simple items still pack
/// multiple-per-line, and only a complex item -- and the item right after
/// it -- force their own line.
///
/// A list of at most one item is printed flat with no group at all: a
/// `group(..)` always expands if its content contains a hard line break
/// anywhere inside, no matter how deeply nested (confirmed via
/// `biome_formatter`'s `propagate_expand`) -- so a single item whose own
/// formatting must hard-break (e.g. a call expression whose arguments
/// don't fit) would otherwise force the keyword itself onto its own line
/// too, even though "does this comma list of items wrap" isn't a
/// meaningful question with only one item.
///
/// Unlike [write_bracketed_fill_list] there's no closing bracket to anchor
/// indentation to, so the expanded variant uses `soft_line_indent_or_space`
/// (leading break+indent only, no trailing one) instead of
/// `soft_block_indent`.
///
/// `is_complex` decides each list item's complexity from its node; callers
/// adapt [is_simple_expression] to whatever expression the list's item
/// wrapper node actually holds (e.g. unwrapping a `SqlSelectExpression`'s
/// `expr` or a `SqlOrderByExpression`'s `item`, ignoring the alias/order-
/// direction suffix since that doesn't affect complexity).
pub(crate) fn write_wrapping_fill_clause<K, L>(
    keyword: K,
    list: &L,
    is_complex: impl Fn(&L::Node) -> bool,
    f: &mut SqlFormatter,
) -> FormatResult<()>
where
    K: Format<SqlFormatContext>,
    L: FormatAstSeparatedListExtension + AsFormat<SqlFormatContext>,
    L::Node: AsFormat<SqlFormatContext> + 'static,
{
    if list.len() <= 1 {
        return write!(f, [keyword, space(), list.format()]);
    }

    let keyword = keyword.memoized();

    let entries: Vec<_> = list
        .iter()
        .zip(
            list.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit),
        )
        .map(|(element, formatted)| {
            let (complex, width) = match &element {
                Ok(node) => (is_complex(node), formatted_width(node, f)),
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
                format_with(|f: &mut SqlFormatter| {
                    f.join_with(space())
                        .entries(entries.iter().map(|(_, _, formatted)| formatted))
                        .finish()
                }),
            ],
            format_args![
                keyword,
                group(&soft_line_indent_or_space(&format_with(
                    |f: &mut SqlFormatter| write_balanced_fill_entries(
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
                    |f: &mut SqlFormatter| write_fill_expression_entries(&entries, f)
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
pub(crate) fn write_wrapping_separated_list<L>(list: &L, f: &mut SqlFormatter) -> FormatResult<()>
where
    L: FormatAstSeparatedListExtension,
    L::Node: AsFormat<SqlFormatContext> + 'static,
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
pub(crate) fn is_simple_expression(expr: &AnySqlExpression, depth: u8) -> bool {
    if depth >= 2 {
        return false;
    }

    match expr {
        AnySqlExpression::AnySqlLiteralExpression(_)
        | AnySqlExpression::SqlName(_)
        | AnySqlExpression::SqlColReference(_)
        | AnySqlExpression::SqlTableColReference(_)
        | AnySqlExpression::SqlStar(_) => true,
        AnySqlExpression::SqlUnaryExpression(unary) => unary
            .expression()
            .is_ok_and(|inner| is_simple_expression(&inner, depth)),
        AnySqlExpression::SqlCallExpression(call) => {
            let arguments = call.arguments();
            arguments.len() as u8 + depth <= 2
                && arguments
                    .iter()
                    .all(|arg| arg.is_ok_and(|arg| is_simple_expression(&arg, depth + 1)))
        }
        AnySqlExpression::PsqlArrayExpression(array) => array
            .items()
            .iter()
            .all(|item| item.is_ok_and(|item| is_simple_expression(&item, depth + 1))),
        _ => false,
    }
}

/// Returns `true` for a `FROM`/`USING` item complex enough to force its own
/// line in a fill layout rather than packing next to others -- anything
/// with a `JOIN` attached, or whose source is a function call, subquery, or
/// parenthesized join rather than a plain table reference. Shared by
/// `from_clause.rs`, `update_from_clause.rs`, and (psql)
/// `delete_using_clause.rs`, all three built on `SqlFromItemList`.
pub(crate) fn is_complex_from_item(item: &SqlFromItem) -> bool {
    if !item.joins().is_empty() {
        return true;
    }

    !matches!(item.source(), Ok(AnySqlFromExpression::SqlTableBinding(_)))
}

/// Prints `l_paren <list> r_paren` (a call's arguments, `INSERT ...
/// VALUES (...)`, an array literal's `[...]`, or any other comma-separated
/// list whose surrounding parens/brackets belong to the *enclosing* node)
/// using a `best_fitting!` with three variants, mirroring `mlang_formatter`'s
/// real call-argument layout (`mlang_formatter/src/rules/expressions/call_arguments.rs`,
/// the `else` branch building `flat_slice`/`expanded_slice`) rather than the
/// simpler always-one-per-line-when-broken [write_wrapping_separated_list]
/// used elsewhere in this formatter:
///
/// 1. Flat: every item on one line, space-separated, no breaks at all.
/// 2. Balanced (tried only if variant 1 doesn't fit): the same complexity-
///    aware breaks as variant 3, but redistributed so lines are as close to
///    equal width as possible instead of greedily cramming each line to the
///    limit -- see [balanced_fill_breaks] for the algorithm and why it's
///    safe as a non-final `best_fitting!` candidate.
/// 3. Expanded (guaranteed-safe fallback if neither above fits): a real
///    `f.fill()` layout where simple items (per the caller's own `is_complex`)
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
///
/// `is_complex` decides each item's complexity from its node, same as
/// [write_wrapping_fill_clause]'s own parameter of the same name -- callers
/// on a plain name/definition list (no meaningful "is this complex"
/// question) can just pass `|_| false`.
pub(crate) fn write_bracketed_fill_list<L>(
    l_paren: SyntaxResult<SqlSyntaxToken>,
    list: &L,
    r_paren: SyntaxResult<SqlSyntaxToken>,
    is_complex: impl Fn(&L::Node) -> bool,
    f: &mut SqlFormatter,
) -> FormatResult<()>
where
    L: FormatAstSeparatedListExtension + AsFormat<SqlFormatContext>,
    L::Node: AsFormat<SqlFormatContext> + 'static,
{
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
            let (complex, width) = match &element {
                Ok(node) => (is_complex(node), formatted_width(node, f)),
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
                l_paren,
                format_with(|f: &mut SqlFormatter| {
                    f.join_with(space())
                        .entries(entries.iter().map(|(_, _, formatted)| formatted))
                        .finish()
                }),
                r_paren,
            ],
            format_args![
                l_paren,
                group(&soft_block_indent(&format_with(|f: &mut SqlFormatter| {
                    write_balanced_fill_entries(&entries, &balanced_breaks, f)
                })))
                .should_expand(true),
                r_paren,
            ],
            format_args![
                l_paren,
                group(&soft_block_indent(&format_with(|f: &mut SqlFormatter| {
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
    f: &mut SqlFormatter,
) -> FormatResult<()>
where
    T: Format<SqlFormatContext>,
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

/// `node`'s own printed width, used by [balanced_fill_breaks] to decide
/// which items pack onto the same line. Must be the *formatted* width, not
/// raw source length -- the latter broke idempotence for any item whose
/// formatting changes its length. Only consulted for "simple" items (see
/// [is_simple_expression]), so the extra format+print pass stays cheap.
fn formatted_width<N>(node: &N, f: &mut SqlFormatter) -> usize
where
    N: AsFormat<SqlFormatContext>,
{
    // Throwaway pass purely to measure width; its output is never emitted,
    // so it must not count against biome_formatter's "every token printed
    // once" tracking, or the real write later on would panic as a duplicate.
    let was_disabled = f.state().is_token_tracking_disabled();
    f.state_mut().set_token_tracking_disabled(true);

    let mut buffer = VecBuffer::new(f.state_mut());
    let write_result = write!(buffer, [node.format()]);
    let document = write_result
        .is_ok()
        .then(|| Document::from(buffer.into_vec()));

    f.state_mut().set_token_tracking_disabled(was_disabled);

    let Some(document) = document else {
        return 0;
    };
    let formatted = Formatted::new(document, f.context().clone());
    // Byte length, matching the measure this replaces -- stays scoped to
    // "source vs formatted length", not byte-vs-char-count for non-ASCII.
    formatted
        .print()
        .map(|printed| printed.as_code().len())
        .unwrap_or(0)
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
    // Only simple items' widths bound the search floor -- a complex item
    // always gets its own forced line regardless of budget, so its own
    // "width" never needs to fit one. That matters because a complex item
    // whose *own* formatting is itself multi-line reports a `formatted_width`
    // that's the summed length of all of its own lines (see its doc comment),
    // easily far exceeding `line_width`. Folding that bogus width into this
    // max would push `lo` above `line_width`, making `lo == hi` short-circuit
    // the search below at that inflated budget -- silently disabling the
    // width limit for every *other* (genuinely simple) item in the list too.
    let max_item_width = entries
        .iter()
        .filter(|(complex, _, _)| !complex)
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
    f: &mut SqlFormatter,
) -> FormatResult<()>
where
    T: Format<SqlFormatContext>,
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
