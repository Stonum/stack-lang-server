use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWithGrantOption;
use sql_syntax::SqlWithGrantOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWithGrantOption;
impl FormatNodeRule<SqlWithGrantOption> for FormatSqlWithGrantOption {
    fn fmt_fields(&self, node: &SqlWithGrantOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlWithGrantOptionFields {
            with_token,
            grant_token,
            option_token,
        } = node.as_fields();

        write!(
            f,
            [
                with_token.format(),
                space(),
                grant_token.format(),
                space(),
                option_token.format()
            ]
        )
    }
}
