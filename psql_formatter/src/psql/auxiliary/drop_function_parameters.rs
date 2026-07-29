use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDropFunctionParameters;
use psql_syntax::PsqlDropFunctionParametersFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropFunctionParameters;
impl FormatNodeRule<PsqlDropFunctionParameters> for FormatPsqlDropFunctionParameters {
    fn fmt_fields(
        &self,
        node: &PsqlDropFunctionParameters,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDropFunctionParametersFields {
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
