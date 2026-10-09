use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSetSchemaAction;
use sql_syntax::SqlSetSchemaActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetSchemaAction;
impl FormatNodeRule<SqlSetSchemaAction> for FormatSqlSetSchemaAction {
    fn fmt_fields(&self, node: &SqlSetSchemaAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetSchemaActionFields {
            set_token,
            schema_token,
            schema,
        } = node.as_fields();

        write!(
            f,
            [
                set_token.format(),
                space(),
                schema_token.format(),
                space(),
                schema.format()
            ]
        )
    }
}
