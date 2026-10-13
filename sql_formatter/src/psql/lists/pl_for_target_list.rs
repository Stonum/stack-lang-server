use crate::prelude::*;
use sql_syntax::PsqlPlForTargetList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForTargetList;
impl FormatRule<PsqlPlForTargetList> for FormatPsqlPlForTargetList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlForTargetList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
