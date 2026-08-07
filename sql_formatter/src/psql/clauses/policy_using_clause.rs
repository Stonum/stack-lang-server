use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPolicyUsingClause;
use sql_syntax::PsqlPolicyUsingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPolicyUsingClause;
impl FormatNodeRule<PsqlPolicyUsingClause> for FormatPsqlPolicyUsingClause {
    fn fmt_fields(&self, node: &PsqlPolicyUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPolicyUsingClauseFields {
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
