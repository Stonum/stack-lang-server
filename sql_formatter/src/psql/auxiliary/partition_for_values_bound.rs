use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPartitionForValuesBound;
use sql_syntax::PsqlPartitionForValuesBoundFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionForValuesBound;
impl FormatNodeRule<PsqlPartitionForValuesBound> for FormatPsqlPartitionForValuesBound {
    fn fmt_fields(
        &self,
        node: &PsqlPartitionForValuesBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPartitionForValuesBoundFields {
            for_token,
            values_token,
            values,
        } = node.as_fields();

        write!(
            f,
            [
                for_token.format(),
                space(),
                values_token.format(),
                space(),
                values.format()
            ]
        )
    }
}
