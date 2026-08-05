use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogusExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogusExpression;
impl FormatBogusNodeRule<SqlBogusExpression> for FormatSqlBogusExpression {}
