use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPolicyUsingClause;
use sql_syntax::SqlPolicyUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPolicyUsingClause;
impl FormatNodeRule<SqlPolicyUsingClause> for FormatSqlPolicyUsingClause {
    fn fmt_fields(&self, node: &SqlPolicyUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPolicyUsingClauseFields {
            using_token,
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                using_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&condition.format())),
                r_paren_token.format(),
            ]
        )
    }
}
