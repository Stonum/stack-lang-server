use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MArrayAssignmentPatternRestElement;
use mlang_syntax::MArrayAssignmentPatternRestElementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMArrayAssignmentPatternRestElement;
impl_format_with_rule!(
    MArrayAssignmentPatternRestElement,
    FormatMArrayAssignmentPatternRestElement
);

impl FormatNodeRule<MArrayAssignmentPatternRestElement>
    for FormatMArrayAssignmentPatternRestElement
{
    fn fmt_fields(
        &self,
        node: &MArrayAssignmentPatternRestElement,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), pattern.format()]]
    }
}
