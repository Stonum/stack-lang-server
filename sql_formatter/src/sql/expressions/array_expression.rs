use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use sql_syntax::SqlArrayExpression;
use sql_syntax::SqlArrayExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlArrayExpression;
impl FormatNodeRule<SqlArrayExpression> for FormatSqlArrayExpression {
    fn fmt_fields(&self, node: &SqlArrayExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlArrayExpressionFields {
            array_token,
            l_brack_token,
            items,
            r_brack_token,
        } = node.as_fields();

        write!(f, [array_token.format()])?;
        write_bracketed_fill_list(l_brack_token, &items, r_brack_token, f)
    }
}
