use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropColumnNotNull;
use sql_syntax::SqlDropColumnNotNullFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropColumnNotNull;
impl FormatNodeRule<SqlDropColumnNotNull> for FormatSqlDropColumnNotNull {
    fn fmt_fields(&self, node: &SqlDropColumnNotNull, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropColumnNotNullFields {
            drop_token,
            not_token,
            null_token,
        } = node.as_fields();

        write!(
            f,
            [
                drop_token.format(),
                space(),
                not_token.format(),
                space(),
                null_token.format()
            ]
        )
    }
}
