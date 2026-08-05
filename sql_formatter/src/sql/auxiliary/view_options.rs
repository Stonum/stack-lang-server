use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlViewOptions;
use sql_syntax::SqlViewOptionsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlViewOptions;
impl FormatNodeRule<SqlViewOptions> for FormatSqlViewOptions {
    fn fmt_fields(&self, node: &SqlViewOptions, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlViewOptionsFields {
            with_token,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                with_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&items.format())),
                r_paren_token.format(),
            ]
        )
    }
}
