use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReturnsTableColumn;
use sql_syntax::SqlReturnsTableColumnFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsTableColumn;
impl FormatNodeRule<SqlReturnsTableColumn> for FormatSqlReturnsTableColumn {
    fn fmt_fields(&self, node: &SqlReturnsTableColumn, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturnsTableColumnFields { name, ty } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])
    }
}
