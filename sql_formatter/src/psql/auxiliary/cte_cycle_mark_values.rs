use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCteCycleMarkValues;
use sql_syntax::PsqlCteCycleMarkValuesFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteCycleMarkValues;
impl FormatNodeRule<PsqlCteCycleMarkValues> for FormatPsqlCteCycleMarkValues {
    fn fmt_fields(&self, node: &PsqlCteCycleMarkValues, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCteCycleMarkValuesFields {
            to_token,
            value,
            default_token,
            default,
        } = node.as_fields();

        write!(
            f,
            [
                to_token.format(),
                space(),
                value.format(),
                space(),
                default_token.format(),
                space(),
                default.format()
            ]
        )
    }
}
