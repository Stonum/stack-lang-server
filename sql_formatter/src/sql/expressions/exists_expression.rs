use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlExistsExpression;
use sql_syntax::SqlExistsExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlExistsExpression;
impl FormatNodeRule<SqlExistsExpression> for FormatSqlExistsExpression {
    fn fmt_fields(&self, node: &SqlExistsExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlExistsExpressionFields {
            exists_token,
            subquery,
        } = node.as_fields();

        write!(f, [exists_token.format(), space(), subquery.format()])
    }
}
