use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlScrollOption;
use sql_syntax::PsqlPlScrollOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlScrollOption;
impl FormatNodeRule<PsqlPlScrollOption> for FormatPsqlPlScrollOption {
    fn fmt_fields(&self, node: &PsqlPlScrollOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlScrollOptionFields {
            no_token,
            scroll_token,
        } = node.as_fields();

        if let Some(no_token) = no_token {
            write!(f, [no_token.format(), space()])?;
        }
        write!(f, [scroll_token.format()])
    }
}
