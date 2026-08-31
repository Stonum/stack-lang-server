use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use sql_syntax::PsqlDropFunctionParameters;
use sql_syntax::PsqlDropFunctionParametersFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropFunctionParameters;
impl FormatNodeRule<PsqlDropFunctionParameters> for FormatPsqlDropFunctionParameters {
    fn fmt_fields(
        &self,
        node: &PsqlDropFunctionParameters,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDropFunctionParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, |_| false, f)
    }
}
