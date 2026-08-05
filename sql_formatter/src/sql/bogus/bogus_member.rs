use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogusMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogusMember;
impl FormatBogusNodeRule<SqlBogusMember> for FormatSqlBogusMember {}
