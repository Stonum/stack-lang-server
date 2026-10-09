use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPartitionRangeValues;
use sql_syntax::PsqlPartitionRangeValuesFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionRangeValues;
impl FormatNodeRule<PsqlPartitionRangeValues> for FormatPsqlPartitionRangeValues {
    fn fmt_fields(
        &self,
        node: &PsqlPartitionRangeValues,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPartitionRangeValuesFields {
            from_token,
            from,
            to_token,
            to,
        } = node.as_fields();

        write!(
            f,
            [
                from_token.format(),
                space(),
                from.format(),
                space(),
                to_token.format(),
                space(),
                to.format()
            ]
        )
    }
}
