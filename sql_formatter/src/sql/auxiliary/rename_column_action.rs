use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlRenameColumnAction;
use sql_syntax::SqlRenameColumnActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlRenameColumnAction;
impl FormatNodeRule<SqlRenameColumnAction> for FormatSqlRenameColumnAction {
    fn fmt_fields(&self, node: &SqlRenameColumnAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlRenameColumnActionFields {
            rename_token,
            column_token,
            old_name,
            to_token,
            new_name,
        } = node.as_fields();

        write!(f, [rename_token.format()])?;
        if let Some(column_token) = column_token {
            write!(f, [space(), column_token.format()])?;
        }
        write!(
            f,
            [
                space(),
                old_name.format(),
                space(),
                to_token.format(),
                space(),
                new_name.format()
            ]
        )
    }
}
