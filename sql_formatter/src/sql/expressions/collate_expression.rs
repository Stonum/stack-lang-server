use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCollateExpression;
use sql_syntax::SqlCollateExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCollateExpression;
impl FormatNodeRule<SqlCollateExpression> for FormatSqlCollateExpression {
    fn fmt_fields(&self, node: &SqlCollateExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCollateExpressionFields {
            expression,
            collate_token,
            collation,
        } = node.as_fields();

        write!(
            f,
            [
                expression.format(),
                space(),
                collate_token.format(),
                space(),
                collation.format()
            ]
        )
    }
}
