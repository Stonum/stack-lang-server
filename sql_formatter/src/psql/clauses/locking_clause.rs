use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlLockingClause;
use sql_syntax::PsqlLockingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLockingClause;
impl FormatNodeRule<PsqlLockingClause> for FormatPsqlLockingClause {
    fn fmt_fields(&self, node: &PsqlLockingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlLockingClauseFields {
            for_token,
            no_token,
            key_token,
            strength,
            of_clause,
            wait_policy,
        } = node.as_fields();

        write!(f, [for_token.format()])?;
        if let Some(no_token) = no_token {
            write!(f, [space(), no_token.format()])?;
        }
        if let Some(key_token) = key_token {
            write!(f, [space(), key_token.format()])?;
        }
        write!(f, [space(), strength.format()])?;
        if let Some(of_clause) = of_clause {
            write!(f, [space(), of_clause.format()])?;
        }
        if let Some(wait_policy) = wait_policy {
            write!(f, [space(), wait_policy.format()])?;
        }
        Ok(())
    }
}
