use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use sql_syntax::SqlAliasColumnList;
use sql_syntax::SqlAliasColumnListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAliasColumnList;
impl FormatNodeRule<SqlAliasColumnList> for FormatSqlAliasColumnList {
    fn fmt_fields(&self, node: &SqlAliasColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAliasColumnListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, |_| false, f)
    }
}
