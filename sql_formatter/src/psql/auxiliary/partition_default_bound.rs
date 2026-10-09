use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPartitionDefaultBound;
use sql_syntax::PsqlPartitionDefaultBoundFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionDefaultBound;
impl FormatNodeRule<PsqlPartitionDefaultBound> for FormatPsqlPartitionDefaultBound {
    fn fmt_fields(
        &self,
        node: &PsqlPartitionDefaultBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPartitionDefaultBoundFields { default_token } = node.as_fields();

        write!(f, [default_token.format()])
    }
}
