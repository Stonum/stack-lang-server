use crate::prelude::*;

/// Prints a comma-separated list on a single line, e.g. `a, b, c`. No
/// trailing comma (there's no wrapped line to justify one) and no
/// conditional wrapping -- that's `Point 4`'s job once lists need to break
/// across lines when they don't fit.
pub(crate) fn write_flat_separated_list<L>(list: &L, f: &mut PsqlFormatter) -> FormatResult<()>
where
    L: FormatAstSeparatedListExtension,
    L::Node: AsFormat<PsqlFormatContext> + 'static,
{
    f.join_with(space())
        .entries(
            list.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit),
        )
        .finish()
}
