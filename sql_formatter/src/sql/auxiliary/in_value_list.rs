use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use sql_syntax::AnySqlExpression;
use sql_syntax::SqlInValueList;
use sql_syntax::SqlInValueListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlInValueList;
impl FormatNodeRule<SqlInValueList> for FormatSqlInValueList {
    fn fmt_fields(&self, node: &SqlInValueList, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlInValueListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(
            l_paren_token,
            &items,
            r_paren_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
