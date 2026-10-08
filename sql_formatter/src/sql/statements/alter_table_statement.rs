use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::SqlAlterTableStatement;
use sql_syntax::SqlAlterTableStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlterTableStatement;
impl FormatNodeRule<SqlAlterTableStatement> for FormatSqlAlterTableStatement {
    fn fmt_fields(&self, node: &SqlAlterTableStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAlterTableStatementFields {
            alter_token,
            table_token,
            if_token,
            exists_token,
            only_token,
            name,
            actions,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [alter_token.format(), space(), table_token.format()])?;
            for token in [&if_token, &exists_token, &only_token]
                .into_iter()
                .flatten()
            {
                write!(f, [space(), token.format()])?;
            }
            write!(f, [space(), name.format()])
        });

        write_wrapping_fill_clause(keyword, &actions, |_| false, f)?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
