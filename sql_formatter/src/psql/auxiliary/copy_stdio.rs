use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCopyStdio;
use sql_syntax::PsqlCopyStdioFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyStdio;
impl FormatNodeRule<PsqlCopyStdio> for FormatPsqlCopyStdio {
    fn fmt_fields(&self, node: &PsqlCopyStdio, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCopyStdioFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
