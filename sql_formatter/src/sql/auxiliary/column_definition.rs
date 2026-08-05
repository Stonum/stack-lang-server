use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlColumnDefinition;
use sql_syntax::SqlColumnDefinitionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnDefinition;
impl FormatNodeRule<SqlColumnDefinition> for FormatSqlColumnDefinition {
    fn fmt_fields(&self, node: &SqlColumnDefinition, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlColumnDefinitionFields { name, ty } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])
    }
}
