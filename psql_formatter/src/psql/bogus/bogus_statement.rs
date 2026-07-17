use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogusStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogusStatement;
impl FormatBogusNodeRule<PsqlBogusStatement> for FormatPsqlBogusStatement {}
