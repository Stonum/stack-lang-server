use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlExceptionSection;
use sql_syntax::PsqlPlExceptionSectionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExceptionSection;
impl FormatNodeRule<PsqlPlExceptionSection> for FormatPsqlPlExceptionSection {
    fn fmt_fields(&self, node: &PsqlPlExceptionSection, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlExceptionSectionFields {
            exception_token,
            handlers,
        } = node.as_fields();

        write!(f, [exception_token.format()])?;
        if handlers.is_empty() {
            return write!(f, [hard_line_break()]);
        }
        write!(f, [block_indent(&handlers.format())])
    }
}
