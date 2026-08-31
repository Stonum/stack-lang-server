use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use sql_syntax::SqlColumnList;
use sql_syntax::SqlColumnListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnList;
impl FormatNodeRule<SqlColumnList> for FormatSqlColumnList {
    fn fmt_fields(&self, node: &SqlColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlColumnListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, |_| false, f)
    }
}
