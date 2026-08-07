use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPolicyForClause;
use sql_syntax::PsqlPolicyForClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPolicyForClause;
impl FormatNodeRule<PsqlPolicyForClause> for FormatPsqlPolicyForClause {
    fn fmt_fields(&self, node: &PsqlPolicyForClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPolicyForClauseFields { for_token, command } = node.as_fields();

        write!(f, [for_token.format(), space(), command.format()])
    }
}
