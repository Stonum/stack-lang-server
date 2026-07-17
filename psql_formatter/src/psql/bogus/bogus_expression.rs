use crate::FormatBogusNodeRule;
use psql_syntax::PsqlBogusExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlBogusExpression;
impl FormatBogusNodeRule<PsqlBogusExpression> for FormatPsqlBogusExpression {}
