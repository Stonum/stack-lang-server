use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MObjectAssignmentPatternShorthandProperty;
use mlang_syntax::MObjectAssignmentPatternShorthandPropertyFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectAssignmentPatternShorthandProperty;
impl_format_with_rule!(
    MObjectAssignmentPatternShorthandProperty,
    FormatMObjectAssignmentPatternShorthandProperty
);

impl FormatNodeRule<MObjectAssignmentPatternShorthandProperty>
    for FormatMObjectAssignmentPatternShorthandProperty
{
    fn fmt_fields(
        &self,
        node: &MObjectAssignmentPatternShorthandProperty,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MObjectAssignmentPatternShorthandPropertyFields { identifier } = node.as_fields();

        write![f, [identifier.format()]]
    }
}
