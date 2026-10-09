use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAlterIndexStatement;
use sql_syntax::SqlAlterIndexStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlterIndexStatement;
impl FormatNodeRule<SqlAlterIndexStatement> for FormatSqlAlterIndexStatement {
    fn fmt_fields(&self, node: &SqlAlterIndexStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAlterIndexStatementFields {
            alter_token,
            index_token,
            if_token,
            exists_token,
            name,
            action,
            semicolon_token,
        } = node.as_fields();

        write!(f, [alter_token.format(), space(), index_token.format()])?;
        for token in [&if_token, &exists_token].into_iter().flatten() {
            write!(f, [space(), token.format()])?;
        }
        // An action that doesn't fit moves under the index name.
        write!(
            f,
            [
                space(),
                name.format(),
                group(&soft_line_indent_or_space(&action.format()))
            ]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
