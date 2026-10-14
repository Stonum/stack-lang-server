use crate::prelude::*;
use sql_syntax::PsqlXmlNamespaceList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlNamespaceList;
impl FormatRule<PsqlXmlNamespaceList> for FormatPsqlXmlNamespaceList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlXmlNamespaceList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
