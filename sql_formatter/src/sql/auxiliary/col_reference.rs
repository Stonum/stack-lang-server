use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlColReference;
use sql_syntax::SqlColReferenceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColReference;
impl FormatNodeRule<SqlColReference> for FormatSqlColReference {
    fn fmt_fields(&self, node: &SqlColReference, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlColReferenceFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
