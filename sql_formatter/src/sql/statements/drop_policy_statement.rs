use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropPolicyStatement;
use sql_syntax::SqlDropPolicyStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropPolicyStatement;
impl FormatNodeRule<SqlDropPolicyStatement> for FormatSqlDropPolicyStatement {
    fn fmt_fields(&self, node: &SqlDropPolicyStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropPolicyStatementFields {
            drop_token,
            policy_token,
            if_token,
            exists_token,
            name,
            on_token,
            table,
            semicolon_token,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), policy_token.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(exists_token) = exists_token {
            write!(f, [space(), exists_token.format()])?;
        }

        write!(
            f,
            [
                space(),
                name.format(),
                space(),
                on_token.format(),
                space(),
                table.format(),
            ]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
