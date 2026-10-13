use crate::prelude::*;
use sql_syntax::PsqlPlRaiseOptionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlRaiseOptionList;
impl FormatRule<PsqlPlRaiseOptionList> for FormatPsqlPlRaiseOptionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlRaiseOptionList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
