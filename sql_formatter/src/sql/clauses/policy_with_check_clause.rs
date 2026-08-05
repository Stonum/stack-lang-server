use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPolicyWithCheckClause;
use sql_syntax::SqlPolicyWithCheckClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPolicyWithCheckClause;
impl FormatNodeRule<SqlPolicyWithCheckClause> for FormatSqlPolicyWithCheckClause {
    fn fmt_fields(
        &self,
        node: &SqlPolicyWithCheckClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlPolicyWithCheckClauseFields {
            with_token,
            check_token,
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                with_token.format(),
                space(),
                check_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&condition.format())),
                r_paren_token.format(),
            ]
        )
    }
}
