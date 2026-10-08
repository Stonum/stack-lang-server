use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCallStatement;
use sql_syntax::SqlCallStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCallStatement;
impl FormatNodeRule<SqlCallStatement> for FormatSqlCallStatement {
    fn fmt_fields(&self, node: &SqlCallStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCallStatementFields {
            call_token,
            procedure,
            semicolon_token,
        } = node.as_fields();

        write!(f, [call_token.format(), space(), procedure.format()])?;
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
