use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSetColumnDefault;
use sql_syntax::SqlSetColumnDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetColumnDefault;
impl FormatNodeRule<SqlSetColumnDefault> for FormatSqlSetColumnDefault {
    fn fmt_fields(&self, node: &SqlSetColumnDefault, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetColumnDefaultFields {
            set_token,
            default_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                set_token.format(),
                space(),
                default_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
