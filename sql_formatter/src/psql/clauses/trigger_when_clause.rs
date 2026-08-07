use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTriggerWhenClause;
use sql_syntax::PsqlTriggerWhenClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerWhenClause;
impl FormatNodeRule<PsqlTriggerWhenClause> for FormatPsqlTriggerWhenClause {
    fn fmt_fields(&self, node: &PsqlTriggerWhenClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlTriggerWhenClauseFields {
            when_token,
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                when_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&condition.format())),
                r_paren_token.format(),
            ]
        )
    }
}
