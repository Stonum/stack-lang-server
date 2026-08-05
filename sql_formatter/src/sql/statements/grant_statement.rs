use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlGrantStatement;
use sql_syntax::SqlGrantStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlGrantStatement;
impl FormatNodeRule<SqlGrantStatement> for FormatSqlGrantStatement {
    fn fmt_fields(&self, node: &SqlGrantStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlGrantStatementFields {
            grant_token,
            all_token,
            on_token,
            table_token,
            objects,
            to_token,
            grantees,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                grant_token.format(),
                space(),
                all_token.format(),
                space(),
                on_token.format(),
            ]
        )?;

        if let Some(table_token) = table_token {
            write!(f, [space(), table_token.format()])?;
        }

        if objects.len() <= 1 {
            write!(f, [space(), objects.format()])?;
        } else {
            write!(f, [group(&soft_line_indent_or_space(&objects.format()))])?;
        }

        write!(f, [space(), to_token.format()])?;

        if grantees.len() <= 1 {
            write!(f, [space(), grantees.format()])?;
        } else {
            write!(f, [group(&soft_line_indent_or_space(&grantees.format()))])?;
        }

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
