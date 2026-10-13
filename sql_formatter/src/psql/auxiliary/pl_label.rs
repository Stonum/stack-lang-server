use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlLabel;
use sql_syntax::PsqlPlLabelFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlLabel;
impl FormatNodeRule<PsqlPlLabel> for FormatPsqlPlLabel {
    fn fmt_fields(&self, node: &PsqlPlLabel, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlLabelFields {
            left_shift_token,
            name,
            right_shift_token,
        } = node.as_fields();

        write!(
            f,
            [
                left_shift_token.format(),
                name.format(),
                right_shift_token.format()
            ]
        )
    }
}
