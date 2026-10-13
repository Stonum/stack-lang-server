use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlForeachSlice;
use sql_syntax::PsqlPlForeachSliceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForeachSlice;
impl FormatNodeRule<PsqlPlForeachSlice> for FormatPsqlPlForeachSlice {
    fn fmt_fields(&self, node: &PsqlPlForeachSlice, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlForeachSliceFields {
            slice_token,
            dimension,
        } = node.as_fields();

        write!(f, [slice_token.format(), space(), dimension.format()])
    }
}
