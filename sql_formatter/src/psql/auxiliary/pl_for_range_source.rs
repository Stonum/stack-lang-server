use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlForRangeSource;
use sql_syntax::PsqlPlForRangeSourceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForRangeSource;
impl FormatNodeRule<PsqlPlForRangeSource> for FormatPsqlPlForRangeSource {
    fn fmt_fields(&self, node: &PsqlPlForRangeSource, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlForRangeSourceFields {
            reverse_token,
            lower,
            dot2_token,
            upper,
            step,
        } = node.as_fields();

        if let Some(reverse_token) = reverse_token {
            write!(f, [reverse_token.format(), space()])?;
        }
        write!(f, [lower.format(), dot2_token.format(), upper.format()])?;
        if let Some(step) = step {
            write!(f, [space(), step.format()])?;
        }
        Ok(())
    }
}
