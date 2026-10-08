use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCreateTableAsStatement;
use sql_syntax::SqlCreateTableAsStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCreateTableAsStatement;
impl FormatNodeRule<SqlCreateTableAsStatement> for FormatSqlCreateTableAsStatement {
    fn fmt_fields(
        &self,
        node: &SqlCreateTableAsStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlCreateTableAsStatementFields {
            create_token,
            persistence,
            table_token,
            if_token,
            not_token,
            exists_token,
            name,
            columns,
            on_commit,
            as_token,
            query,
            with_data,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format()])?;
        if let Some(persistence) = persistence {
            write!(f, [space(), persistence.format()])?;
        }
        write!(f, [space(), table_token.format()])?;
        for token in [&if_token, &not_token, &exists_token].into_iter().flatten() {
            write!(f, [space(), token.format()])?;
        }
        write!(f, [space(), name.format()])?;

        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }
        if let Some(on_commit) = on_commit {
            write!(f, [space(), on_commit.format()])?;
        }

        write!(f, [space(), as_token.format(), space(), query.format()])?;

        if let Some(with_data) = with_data {
            write!(f, [space(), with_data.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
