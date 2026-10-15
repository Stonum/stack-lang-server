use crate::prelude::*;
use crate::utils::{is_simple_call_argument, write_bracketed_fill_list};
use sql_syntax::AnySqlCallArgument;
use sql_syntax::PsqlPlOpenArguments;
use sql_syntax::PsqlPlOpenArgumentsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlOpenArguments;
impl FormatNodeRule<PsqlPlOpenArguments> for FormatPsqlPlOpenArguments {
    fn fmt_fields(&self, node: &PsqlPlOpenArguments, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlOpenArgumentsFields {
            l_paren_token,
            arguments,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(
            l_paren_token,
            &arguments,
            r_paren_token,
            |argument: &AnySqlCallArgument| !is_simple_call_argument(argument, 0),
            f,
        )
    }
}
