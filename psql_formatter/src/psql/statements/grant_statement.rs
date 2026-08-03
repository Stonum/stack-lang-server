use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlGrantStatement;
use psql_syntax::PsqlGrantStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlGrantStatement;
impl FormatNodeRule<PsqlGrantStatement> for FormatPsqlGrantStatement {
    fn fmt_fields(&self, node: &PsqlGrantStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlGrantStatementFields {
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
