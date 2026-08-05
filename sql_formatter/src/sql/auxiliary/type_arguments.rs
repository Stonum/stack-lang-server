use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTypeArguments;
use sql_syntax::SqlTypeArgumentsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTypeArguments;
impl FormatNodeRule<SqlTypeArguments> for FormatSqlTypeArguments {
    fn fmt_fields(&self, node: &SqlTypeArguments, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTypeArgumentsFields {
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
