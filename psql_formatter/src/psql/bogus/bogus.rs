use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogus;
impl FormatBogusNodeRule<PsqlBogus> for FormatPsqlBogus {}
