use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlPolicyForClause;
use psql_syntax::PsqlPolicyForClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPolicyForClause;
impl FormatNodeRule<PsqlPolicyForClause> for FormatPsqlPolicyForClause {
    fn fmt_fields(&self, node: &PsqlPolicyForClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlPolicyForClauseFields { for_token, command } = node.as_fields();

        write!(f, [for_token.format(), space(), command.format()])
    }
}
