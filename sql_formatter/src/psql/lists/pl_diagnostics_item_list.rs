use crate::prelude::*;
use sql_syntax::PsqlPlDiagnosticsItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlDiagnosticsItemList;
impl FormatRule<PsqlPlDiagnosticsItemList> for FormatPsqlPlDiagnosticsItemList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlDiagnosticsItemList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
