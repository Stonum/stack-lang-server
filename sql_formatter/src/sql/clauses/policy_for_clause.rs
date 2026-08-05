use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPolicyForClause;
use sql_syntax::SqlPolicyForClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPolicyForClause;
impl FormatNodeRule<SqlPolicyForClause> for FormatSqlPolicyForClause {
    fn fmt_fields(&self, node: &SqlPolicyForClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPolicyForClauseFields { for_token, command } = node.as_fields();

        write!(f, [for_token.format(), space(), command.format()])
    }
}
