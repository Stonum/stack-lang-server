use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTableName;
use sql_syntax::SqlTableNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableName;
impl FormatNodeRule<SqlTableName> for FormatSqlTableName {
    fn fmt_fields(&self, node: &SqlTableName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTableNameFields { schema, name } = node.as_fields();

        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])
    }
}
