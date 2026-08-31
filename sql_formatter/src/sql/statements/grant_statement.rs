use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
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

        let objects_keyword = format_with(move |f| {
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
            if let Some(table_token) = &table_token {
                write!(f, [space(), table_token.format()])?;
            }
            Ok(())
        });
        write_wrapping_fill_clause(objects_keyword, &objects, |_| false, f)?;

        write!(f, [space()])?;
        write_wrapping_fill_clause(to_token.format(), &grantees, |_| false, f)?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
