use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlGoStatement;
use sql_syntax::SqlGoStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlGoStatement;
impl FormatNodeRule<SqlGoStatement> for FormatSqlGoStatement {
    fn fmt_fields(&self, node: &SqlGoStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlGoStatementFields {
            go_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [go_token.format()])?;
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
