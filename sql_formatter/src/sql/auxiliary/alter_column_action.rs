use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAlterColumnAction;
use sql_syntax::SqlAlterColumnActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlterColumnAction;
impl FormatNodeRule<SqlAlterColumnAction> for FormatSqlAlterColumnAction {
    fn fmt_fields(&self, node: &SqlAlterColumnAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAlterColumnActionFields {
            alter_token,
            column_token,
            name,
            operation,
        } = node.as_fields();

        write!(f, [alter_token.format()])?;
        if let Some(column_token) = column_token {
            write!(f, [space(), column_token.format()])?;
        }
        write!(f, [space(), name.format(), space(), operation.format()])
    }
}
