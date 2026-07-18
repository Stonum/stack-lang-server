use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTypeArguments;
use psql_syntax::PsqlTypeArgumentsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeArguments;
impl FormatNodeRule<PsqlTypeArguments> for FormatPsqlTypeArguments {
    fn fmt_fields(&self, node: &PsqlTypeArguments, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTypeArgumentsFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&items.format())),
                r_paren_token.format(),
            ]
        )
    }
}
