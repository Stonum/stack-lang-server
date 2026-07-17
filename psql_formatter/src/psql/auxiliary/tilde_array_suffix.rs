use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTildeArraySuffix;
use psql_syntax::PsqlTildeArraySuffixFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeArraySuffix;
impl FormatNodeRule<PsqlTildeArraySuffix> for FormatPsqlTildeArraySuffix {
    fn fmt_fields(&self, node: &PsqlTildeArraySuffix, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTildeArraySuffixFields {
            open_tilde_token,
            l_brack_token,
            r_brack_token,
            close_tilde_token,
        } = node.as_fields();

        write!(
            f,
            [
                open_tilde_token.format(),
                l_brack_token.format(),
                r_brack_token.format(),
                close_tilde_token.format(),
            ]
        )
    }
}
