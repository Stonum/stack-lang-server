use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use sql_syntax::PsqlCopyOptions;
use sql_syntax::PsqlCopyOptionsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyOptions;
impl FormatNodeRule<PsqlCopyOptions> for FormatPsqlCopyOptions {
    fn fmt_fields(&self, node: &PsqlCopyOptions, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCopyOptionsFields {
            with_token,
            l_paren_token,
            options,
            r_paren_token,
        } = node.as_fields();

        if let Some(with_token) = with_token {
            write!(f, [with_token.format(), space()])?;
        }
        write_bracketed_fill_list(l_paren_token, &options, r_paren_token, |_| false, f)
    }
}
