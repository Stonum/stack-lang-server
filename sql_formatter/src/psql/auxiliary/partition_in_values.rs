use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlPartitionInValues;
use sql_syntax::PsqlPartitionInValuesFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionInValues;
impl FormatNodeRule<PsqlPartitionInValues> for FormatPsqlPartitionInValues {
    fn fmt_fields(&self, node: &PsqlPartitionInValues, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPartitionInValuesFields {
            in_token,
            l_paren_token,
            values,
            r_paren_token,
        } = node.as_fields();

        write!(f, [in_token.format(), space()])?;
        write_bracketed_fill_list(
            l_paren_token,
            &values,
            r_paren_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )
    }
}
