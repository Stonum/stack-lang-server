use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogusParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogusParameter;
impl FormatBogusNodeRule<SqlBogusParameter> for FormatSqlBogusParameter {}
