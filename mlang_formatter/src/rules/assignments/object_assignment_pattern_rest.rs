use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MObjectAssignmentPatternRest;
use mlang_syntax::MObjectAssignmentPatternRestFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectAssignmentPatternRest;
impl_format_with_rule!(
    MObjectAssignmentPatternRest,
    FormatMObjectAssignmentPatternRest
);

impl FormatNodeRule<MObjectAssignmentPatternRest> for FormatMObjectAssignmentPatternRest {
    fn fmt_fields(
        &self,
        node: &MObjectAssignmentPatternRest,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), target.format()]]
    }
}
