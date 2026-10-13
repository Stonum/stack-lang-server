use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlDeclareSection;
use sql_syntax::PsqlPlDeclareSectionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlDeclareSection;
impl FormatNodeRule<PsqlPlDeclareSection> for FormatPsqlPlDeclareSection {
    fn fmt_fields(&self, node: &PsqlPlDeclareSection, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlDeclareSectionFields {
            declare_token,
            declarations,
        } = node.as_fields();

        write!(f, [declare_token.format()])?;
        if declarations.is_empty() {
            return Ok(());
        }
        write!(f, [block_indent(&declarations.format())])
    }
}
