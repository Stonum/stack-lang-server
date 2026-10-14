use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlLockingOfClause;
use sql_syntax::PsqlLockingOfClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLockingOfClause;
impl FormatNodeRule<PsqlLockingOfClause> for FormatPsqlLockingOfClause {
    fn fmt_fields(&self, node: &PsqlLockingOfClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlLockingOfClauseFields { of_token, tables } = node.as_fields();

        write!(f, [of_token.format(), space(), tables.format()])
    }
}
