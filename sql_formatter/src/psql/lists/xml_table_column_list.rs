use crate::prelude::*;
use sql_syntax::PsqlXmlTableColumnList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlTableColumnList;
impl FormatRule<PsqlXmlTableColumnList> for FormatPsqlXmlTableColumnList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlXmlTableColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
