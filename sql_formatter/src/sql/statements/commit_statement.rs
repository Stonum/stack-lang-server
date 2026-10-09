use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCommitStatement;
use sql_syntax::SqlCommitStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCommitStatement;
impl FormatNodeRule<SqlCommitStatement> for FormatSqlCommitStatement {
    fn fmt_fields(&self, node: &SqlCommitStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCommitStatementFields {
            commit_token,
            transaction_kind,
            semicolon_token,
        } = node.as_fields();

        write!(f, [commit_token.format()])?;
        if let Some(transaction_kind) = transaction_kind {
            write!(f, [space(), transaction_kind.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
