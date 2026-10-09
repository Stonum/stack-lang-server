use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlRollbackStatement;
use sql_syntax::SqlRollbackStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlRollbackStatement;
impl FormatNodeRule<SqlRollbackStatement> for FormatSqlRollbackStatement {
    fn fmt_fields(&self, node: &SqlRollbackStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlRollbackStatementFields {
            rollback_token,
            transaction_kind,
            semicolon_token,
        } = node.as_fields();

        write!(f, [rollback_token.format()])?;
        if let Some(transaction_kind) = transaction_kind {
            write!(f, [space(), transaction_kind.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
