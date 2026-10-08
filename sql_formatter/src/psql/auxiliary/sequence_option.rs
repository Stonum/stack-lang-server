use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlSequenceOption;
use sql_syntax::PsqlSequenceOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSequenceOption;
impl FormatNodeRule<PsqlSequenceOption> for FormatPsqlSequenceOption {
    fn fmt_fields(&self, node: &PsqlSequenceOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlSequenceOptionFields {
            name,
            joiner,
            value,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(joiner) = joiner {
            write!(f, [space(), joiner.format()])?;
        }
        write!(f, [space(), value.format()])
    }
}
