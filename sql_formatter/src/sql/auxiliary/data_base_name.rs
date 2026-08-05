use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDataBaseName;
use sql_syntax::SqlDataBaseNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDataBaseName;
impl FormatNodeRule<SqlDataBaseName> for FormatSqlDataBaseName {
    fn fmt_fields(&self, node: &SqlDataBaseName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDataBaseNameFields { name, dot_token } = node.as_fields();

        write!(f, [name.format(), dot_token.format()])
    }
}
