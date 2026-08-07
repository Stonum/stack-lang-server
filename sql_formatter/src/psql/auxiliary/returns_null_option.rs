use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlReturnsNullOption;
use sql_syntax::PsqlReturnsNullOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsNullOption;
impl FormatNodeRule<PsqlReturnsNullOption> for FormatPsqlReturnsNullOption {
    fn fmt_fields(&self, node: &PsqlReturnsNullOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlReturnsNullOptionFields {
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
