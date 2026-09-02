use crate::prelude::*;
use crate::utils::AnyMAssignmentLike;

use biome_formatter::write;
use mlang_syntax::MObjectAssignmentPatternProperty;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectAssignmentPatternProperty;
impl_format_with_rule!(
    MObjectAssignmentPatternProperty,
    FormatMObjectAssignmentPatternProperty
);

impl FormatNodeRule<MObjectAssignmentPatternProperty> for FormatMObjectAssignmentPatternProperty {
    fn fmt_fields(
        &self,
        node: &MObjectAssignmentPatternProperty,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        write![f, [AnyMAssignmentLike::from(node.clone())]]
    }
}
