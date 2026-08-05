use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropFunctionParameters;
use sql_syntax::SqlDropFunctionParametersFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropFunctionParameters;
impl FormatNodeRule<SqlDropFunctionParameters> for FormatSqlDropFunctionParameters {
    fn fmt_fields(
        &self,
        node: &SqlDropFunctionParameters,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlDropFunctionParametersFields {
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
