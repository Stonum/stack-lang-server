use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlViewOption;
use psql_syntax::PsqlViewOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlViewOption;
impl FormatNodeRule<PsqlViewOption> for FormatPsqlViewOption {
    fn fmt_fields(&self, node: &PsqlViewOption, f: &mut PsqlFormatter) -> FormatResult<()> {
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
