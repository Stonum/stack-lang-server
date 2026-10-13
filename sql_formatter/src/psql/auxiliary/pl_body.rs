use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlBody;
use sql_syntax::PsqlPlBodyFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlBody;
impl FormatNodeRule<PsqlPlBody> for FormatPsqlPlBody {
    fn fmt_fields(&self, node: &PsqlPlBody, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlBodyFields {
            l_quote_token,
            block,
            r_quote_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_quote_token.format(),
                hard_line_break(),
                block.format(),
                hard_line_break(),
                r_quote_token.format()
            ]
        )
    }
}
