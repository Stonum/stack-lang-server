use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlViewOptions;
use psql_syntax::PsqlViewOptionsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlViewOptions;
impl FormatNodeRule<PsqlViewOptions> for FormatPsqlViewOptions {
    fn fmt_fields(&self, node: &PsqlViewOptions, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlViewOptionsFields {
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
