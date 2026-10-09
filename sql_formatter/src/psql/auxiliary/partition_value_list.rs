use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlPartitionValueList;
use sql_syntax::PsqlPartitionValueListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionValueList;
impl FormatNodeRule<PsqlPartitionValueList> for FormatPsqlPartitionValueList {
    fn fmt_fields(&self, node: &PsqlPartitionValueList, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPartitionValueListFields {
            l_paren_token,
            values,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(
            l_paren_token,
            &values,
            r_paren_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
