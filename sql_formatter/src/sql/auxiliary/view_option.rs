use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlViewOption;
use sql_syntax::SqlViewOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlViewOption;
impl FormatNodeRule<SqlViewOption> for FormatSqlViewOption {
    fn fmt_fields(&self, node: &SqlViewOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlViewOptionFields {
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
