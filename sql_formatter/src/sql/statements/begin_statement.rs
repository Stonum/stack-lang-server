use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlBeginStatement;
use sql_syntax::SqlBeginStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlBeginStatement;
impl FormatNodeRule<SqlBeginStatement> for FormatSqlBeginStatement {
    fn fmt_fields(&self, node: &SqlBeginStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlBeginStatementFields {
            kind,
            transaction_kind,
            semicolon_token,
        } = node.as_fields();

        write!(f, [kind.format()])?;
        if let Some(transaction_kind) = transaction_kind {
            write!(f, [space(), transaction_kind.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
