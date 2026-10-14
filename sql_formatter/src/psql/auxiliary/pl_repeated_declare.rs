use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlRepeatedDeclare;
use sql_syntax::PsqlPlRepeatedDeclareFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlRepeatedDeclare;
impl FormatNodeRule<PsqlPlRepeatedDeclare> for FormatPsqlPlRepeatedDeclare {
    fn fmt_fields(&self, node: &PsqlPlRepeatedDeclare, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlRepeatedDeclareFields { declare_token } = node.as_fields();

        write!(f, [declare_token.format()])
    }
}
