use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlName;
use sql_syntax::SqlNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlName;
impl FormatNodeRule<SqlName> for FormatSqlName {
    fn fmt_fields(&self, node: &SqlName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlNameFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
