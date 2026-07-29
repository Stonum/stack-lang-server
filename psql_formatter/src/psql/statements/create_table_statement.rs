use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCreateTableStatement;
use psql_syntax::PsqlCreateTableStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreateTableStatement;
impl FormatNodeRule<PsqlCreateTableStatement> for FormatPsqlCreateTableStatement {
    fn fmt_fields(
        &self,
        node: &PsqlCreateTableStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreateTableStatementFields {
            create_token,
            table_token,
            if_token,
            not_token,
            exists_token,
            name,
            l_paren_token,
            columns,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format(), space(), table_token.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(not_token) = not_token {
            write!(f, [space(), not_token.format()])?;
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
                l_paren_token.format(),
                group(&soft_block_indent(&columns.format())),
                r_paren_token.format(),
            ]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
