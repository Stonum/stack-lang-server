use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTriggerWhenClause;
use sql_syntax::SqlTriggerWhenClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerWhenClause;
impl FormatNodeRule<SqlTriggerWhenClause> for FormatSqlTriggerWhenClause {
    fn fmt_fields(&self, node: &SqlTriggerWhenClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTriggerWhenClauseFields {
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
