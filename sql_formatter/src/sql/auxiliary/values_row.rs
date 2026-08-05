use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use sql_syntax::SqlValuesRow;
use sql_syntax::SqlValuesRowFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlValuesRow;
impl FormatNodeRule<SqlValuesRow> for FormatSqlValuesRow {
    fn fmt_fields(&self, node: &SqlValuesRow, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlValuesRowFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, f)
    }
}
