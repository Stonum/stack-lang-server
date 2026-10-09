use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlAttachPartitionAction;
use sql_syntax::PsqlAttachPartitionActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAttachPartitionAction;
impl FormatNodeRule<PsqlAttachPartitionAction> for FormatPsqlAttachPartitionAction {
    fn fmt_fields(
        &self,
        node: &PsqlAttachPartitionAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlAttachPartitionActionFields {
            attach_token,
            partition_token,
            partition,
            bound,
        } = node.as_fields();

        write!(
            f,
            [
                attach_token.format(),
                space(),
                partition_token.format(),
                space(),
                partition.format(),
                space(),
                bound.format()
            ]
        )
    }
}
