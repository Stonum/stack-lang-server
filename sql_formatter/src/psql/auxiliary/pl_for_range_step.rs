use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlForRangeStep;
use sql_syntax::PsqlPlForRangeStepFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForRangeStep;
impl FormatNodeRule<PsqlPlForRangeStep> for FormatPsqlPlForRangeStep {
    fn fmt_fields(&self, node: &PsqlPlForRangeStep, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlForRangeStepFields { by_token, value } = node.as_fields();

        write!(f, [by_token.format(), space(), value.format()])
    }
}
