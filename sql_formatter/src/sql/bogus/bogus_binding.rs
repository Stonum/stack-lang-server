use crate::FormatBogusNodeRule;
use sql_syntax::SqlBogusBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBogusBinding;
impl FormatBogusNodeRule<SqlBogusBinding> for FormatSqlBogusBinding {}
