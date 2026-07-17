use crate::prelude::*;

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
