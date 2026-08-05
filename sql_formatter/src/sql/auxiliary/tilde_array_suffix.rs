use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTildeArraySuffix;
use sql_syntax::SqlTildeArraySuffixFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTildeArraySuffix;
impl FormatNodeRule<SqlTildeArraySuffix> for FormatSqlTildeArraySuffix {
    fn fmt_fields(&self, node: &SqlTildeArraySuffix, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTildeArraySuffixFields {
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
