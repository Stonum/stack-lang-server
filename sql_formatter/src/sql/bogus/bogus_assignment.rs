use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogusAssignment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogusAssignment;
impl FormatBogusNodeRule<SqlBogusAssignment> for FormatSqlBogusAssignment {}
