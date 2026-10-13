use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlExitWhenClause;
use sql_syntax::PsqlPlExitWhenClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExitWhenClause;
impl FormatNodeRule<PsqlPlExitWhenClause> for FormatPsqlPlExitWhenClause {
    fn fmt_fields(&self, node: &PsqlPlExitWhenClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlExitWhenClauseFields {
            when_token,
            condition,
        } = node.as_fields();

        write!(f, [when_token.format(), space(), condition.format()])
    }
}
