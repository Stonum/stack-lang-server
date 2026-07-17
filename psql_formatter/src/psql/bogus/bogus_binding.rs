use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogusBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogusBinding;
impl FormatBogusNodeRule<PsqlBogusBinding> for FormatPsqlBogusBinding {}
