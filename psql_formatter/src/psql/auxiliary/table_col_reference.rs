use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTableColReference;
use psql_syntax::PsqlTableColReferenceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableColReference;
impl FormatNodeRule<PsqlTableColReference> for FormatPsqlTableColReference {
    fn fmt_fields(&self, node: &PsqlTableColReference, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTableColReferenceFields {
            table,
            dot_token,
            name,
        } = node.as_fields();

        write!(f, [table.format(), dot_token.format(), name.format()])
    }
}
