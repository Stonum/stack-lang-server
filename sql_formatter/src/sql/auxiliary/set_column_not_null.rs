use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSetColumnNotNull;
use sql_syntax::SqlSetColumnNotNullFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetColumnNotNull;
impl FormatNodeRule<SqlSetColumnNotNull> for FormatSqlSetColumnNotNull {
    fn fmt_fields(&self, node: &SqlSetColumnNotNull, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetColumnNotNullFields {
            set_token,
            not_token,
            null_token,
        } = node.as_fields();

        write!(
            f,
            [
                set_token.format(),
                space(),
                not_token.format(),
                space(),
                null_token.format()
            ]
        )
    }
}
