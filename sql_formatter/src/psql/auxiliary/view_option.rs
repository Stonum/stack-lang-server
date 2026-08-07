use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlViewOption;
use sql_syntax::PsqlViewOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlViewOption;
impl FormatNodeRule<PsqlViewOption> for FormatPsqlViewOption {
    fn fmt_fields(&self, node: &PsqlViewOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlViewOptionFields {
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
