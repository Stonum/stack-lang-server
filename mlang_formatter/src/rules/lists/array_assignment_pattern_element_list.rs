use crate::prelude::*;
use crate::utils::array::write_array_node;
use mlang_syntax::MArrayAssignmentPatternElementList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMArrayAssignmentPatternElementList;
impl_format!(
    MArrayAssignmentPatternElementList,
    FormatMArrayAssignmentPatternElementList
);

impl FormatRule<MArrayAssignmentPatternElementList> for FormatMArrayAssignmentPatternElementList {
    type Context = MFormatContext;

    fn fmt(
        &self,
        node: &MArrayAssignmentPatternElementList,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        write_array_node(node, f)
    }
}
