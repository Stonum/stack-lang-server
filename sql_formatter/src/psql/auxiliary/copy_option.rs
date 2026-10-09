use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCopyOption;
use sql_syntax::PsqlCopyOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyOption;
impl FormatNodeRule<PsqlCopyOption> for FormatPsqlCopyOption {
    fn fmt_fields(&self, node: &PsqlCopyOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCopyOptionFields { name, value } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(value) = value {
            write!(f, [space(), value.format()])?;
        }
        Ok(())
    }
}
