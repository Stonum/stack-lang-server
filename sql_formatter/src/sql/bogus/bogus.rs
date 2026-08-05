use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogus;
impl FormatBogusNodeRule<SqlBogus> for FormatSqlBogus {}
