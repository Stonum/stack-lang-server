use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReturnsNullOption;
use sql_syntax::SqlReturnsNullOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReturnsNullOption;
impl FormatNodeRule<SqlReturnsNullOption> for FormatSqlReturnsNullOption {
    fn fmt_fields(&self, node: &SqlReturnsNullOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReturnsNullOptionFields {
            returns_token,
            first_null_token,
            on_token,
            second_null_token,
            input_token,
        } = node.as_fields();

        write!(
            f,
            [
                returns_token.format(),
                space(),
                first_null_token.format(),
                space(),
                on_token.format(),
                space(),
                second_null_token.format(),
                space(),
                input_token.format()
            ]
        )
    }
}
