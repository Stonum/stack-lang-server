use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlGrantOptionFor;
use sql_syntax::SqlGrantOptionForFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlGrantOptionFor;
impl FormatNodeRule<SqlGrantOptionFor> for FormatSqlGrantOptionFor {
    fn fmt_fields(&self, node: &SqlGrantOptionFor, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlGrantOptionForFields {
            grant_token,
            option_token,
            for_token,
        } = node.as_fields();

        write!(
            f,
            [
                grant_token.format(),
                space(),
                option_token.format(),
                space(),
                for_token.format()
            ]
        )
    }
}
