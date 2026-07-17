use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlColReference;
use psql_syntax::PsqlColReferenceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColReference;
impl FormatNodeRule<PsqlColReference> for FormatPsqlColReference {
    fn fmt_fields(&self, node: &PsqlColReference, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlColReferenceFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
