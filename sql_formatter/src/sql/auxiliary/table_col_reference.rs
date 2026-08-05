use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTableColReference;
use sql_syntax::SqlTableColReferenceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableColReference;
impl FormatNodeRule<SqlTableColReference> for FormatSqlTableColReference {
    fn fmt_fields(&self, node: &SqlTableColReference, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTableColReferenceFields {
            table,
            dot_token,
            name,
        } = node.as_fields();

        write!(f, [table.format(), dot_token.format(), name.format()])
    }
}
