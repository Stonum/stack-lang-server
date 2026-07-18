use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_rowan::{AstSeparatedList, SyntaxResult};
use psql_syntax::{
    PsqlFromClause, PsqlGroupByClause, PsqlHavingClause, PsqlLanguage, PsqlSelectClause,
    PsqlSyntaxToken, PsqlWhereClause,
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
