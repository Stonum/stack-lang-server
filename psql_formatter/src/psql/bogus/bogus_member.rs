use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogusMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogusMember;
impl FormatBogusNodeRule<PsqlBogusMember> for FormatPsqlBogusMember {}
