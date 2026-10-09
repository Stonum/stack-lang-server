use crate::prelude::*;
use sql_syntax::SqlPrivilegeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrivilegeList;
impl FormatRule<SqlPrivilegeList> for FormatSqlPrivilegeList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlPrivilegeList, f: &mut SqlFormatter) -> FormatResult<()> {
        // Short, fixed-vocabulary list: wrapping is left to the object and
        // grantee lists that follow it.
        f.join_with(space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
