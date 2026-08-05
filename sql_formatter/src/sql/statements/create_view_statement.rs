use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCreateViewStatement;
use sql_syntax::SqlCreateViewStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCreateViewStatement;
impl FormatNodeRule<SqlCreateViewStatement> for FormatSqlCreateViewStatement {
    fn fmt_fields(&self, node: &SqlCreateViewStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCreateViewStatementFields {
            create_token,
            or_token,
            replace_token,
            view_token,
            name,
            options,
            as_token,
            query,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format()])?;

        if let Some(or_token) = or_token {
            write!(f, [space(), or_token.format()])?;
        }
        if let Some(replace_token) = replace_token {
            write!(f, [space(), replace_token.format()])?;
        }

        write!(f, [space(), view_token.format(), space(), name.format()])?;

        if let Some(options) = options {
            write!(f, [space(), options.format()])?;
        }

        write!(f, [space(), as_token.format(), space(), query.format()])?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
