use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPolicyWithCheckClause;
use sql_syntax::PsqlPolicyWithCheckClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPolicyWithCheckClause;
impl FormatNodeRule<PsqlPolicyWithCheckClause> for FormatPsqlPolicyWithCheckClause {
    fn fmt_fields(
        &self,
        node: &PsqlPolicyWithCheckClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPolicyWithCheckClauseFields {
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
