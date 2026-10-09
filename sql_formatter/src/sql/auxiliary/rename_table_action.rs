use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlRenameTableAction;
use sql_syntax::SqlRenameTableActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlRenameTableAction;
impl FormatNodeRule<SqlRenameTableAction> for FormatSqlRenameTableAction {
    fn fmt_fields(&self, node: &SqlRenameTableAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlRenameTableActionFields {
            rename_token,
            to_token,
            new_name,
        } = node.as_fields();

        write!(
            f,
            [
                rename_token.format(),
                space(),
                to_token.format(),
                space(),
                new_name.format()
            ]
        )
    }
}
