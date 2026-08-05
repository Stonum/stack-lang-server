use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogusStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogusStatement;
impl FormatBogusNodeRule<SqlBogusStatement> for FormatSqlBogusStatement {}
