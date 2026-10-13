use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlRaiseOption;
use sql_syntax::PsqlPlRaiseOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlRaiseOption;
impl FormatNodeRule<PsqlPlRaiseOption> for FormatPsqlPlRaiseOption {
    fn fmt_fields(&self, node: &PsqlPlRaiseOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlRaiseOptionFields {
            name,
            eq_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                space(),
                eq_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
