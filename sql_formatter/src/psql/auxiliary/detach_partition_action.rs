use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlDetachPartitionAction;
use sql_syntax::PsqlDetachPartitionActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDetachPartitionAction;
impl FormatNodeRule<PsqlDetachPartitionAction> for FormatPsqlDetachPartitionAction {
    fn fmt_fields(
        &self,
        node: &PsqlDetachPartitionAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDetachPartitionActionFields {
            detach_token,
            partition_token,
            partition,
            mode,
        } = node.as_fields();

        write!(
            f,
            [
                detach_token.format(),
                space(),
                partition_token.format(),
                space(),
                partition.format()
            ]
        )?;
        if let Some(mode) = mode {
            write!(f, [space(), mode.format()])?;
        }
        Ok(())
    }
}
