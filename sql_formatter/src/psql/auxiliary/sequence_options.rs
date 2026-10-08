use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlSequenceOptions;
use sql_syntax::PsqlSequenceOptionsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSequenceOptions;
impl FormatNodeRule<PsqlSequenceOptions> for FormatPsqlSequenceOptions {
    fn fmt_fields(&self, node: &PsqlSequenceOptions, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlSequenceOptionsFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                items.format(),
                r_paren_token.format()
            ]
        )
    }
}
