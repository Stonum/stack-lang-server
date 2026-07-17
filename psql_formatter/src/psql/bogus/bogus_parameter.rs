use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogusParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogusParameter;
impl FormatBogusNodeRule<PsqlBogusParameter> for FormatPsqlBogusParameter {}
