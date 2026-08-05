use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTildeName;
use sql_syntax::SqlTildeNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTildeName;
impl FormatNodeRule<SqlTildeName> for FormatSqlTildeName {
    fn fmt_fields(&self, node: &SqlTildeName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTildeNameFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
