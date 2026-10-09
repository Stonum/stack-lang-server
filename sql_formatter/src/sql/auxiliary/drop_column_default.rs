use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropColumnDefault;
use sql_syntax::SqlDropColumnDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropColumnDefault;
impl FormatNodeRule<SqlDropColumnDefault> for FormatSqlDropColumnDefault {
    fn fmt_fields(&self, node: &SqlDropColumnDefault, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropColumnDefaultFields {
            drop_token,
            default_token,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), default_token.format()])
    }
}
