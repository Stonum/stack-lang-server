use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlRenameConstraintAction;
use sql_syntax::SqlRenameConstraintActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlRenameConstraintAction;
impl FormatNodeRule<SqlRenameConstraintAction> for FormatSqlRenameConstraintAction {
    fn fmt_fields(
        &self,
        node: &SqlRenameConstraintAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlRenameConstraintActionFields {
            rename_token,
            constraint_token,
            old_name,
            to_token,
            new_name,
        } = node.as_fields();

        write!(
            f,
            [
                rename_token.format(),
                space(),
                constraint_token.format(),
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
