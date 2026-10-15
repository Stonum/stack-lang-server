use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use sql_syntax::PsqlPlCursorParameter;
use sql_syntax::PsqlPlCursorParameters;
use sql_syntax::PsqlPlCursorParametersFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCursorParameters;
impl FormatNodeRule<PsqlPlCursorParameters> for FormatPsqlPlCursorParameters {
    fn fmt_fields(&self, node: &PsqlPlCursorParameters, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlCursorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(
            l_paren_token,
            &parameters,
            r_paren_token,
            |_: &PsqlPlCursorParameter| false,
            f,
        )
    }
}
