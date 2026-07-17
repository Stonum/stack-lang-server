use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogusAssignment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogusAssignment;
impl FormatBogusNodeRule<PsqlBogusAssignment> for FormatPsqlBogusAssignment {}
