use crate::prelude::*;
use mlang_syntax::AnyMObjectAssignmentPatternMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMObjectAssignmentPatternMember;
impl_format!(
    AnyMObjectAssignmentPatternMember,
    FormatAnyMObjectAssignmentPatternMember
);

impl FormatRule<AnyMObjectAssignmentPatternMember> for FormatAnyMObjectAssignmentPatternMember {
    type Context = MFormatContext;
    fn fmt(
        &self,
        node: &AnyMObjectAssignmentPatternMember,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyMObjectAssignmentPatternMember::MBogusAssignment(node) => node.format().fmt(f),
            AnyMObjectAssignmentPatternMember::MObjectAssignmentPatternProperty(node) => {
                node.format().fmt(f)
            }
            AnyMObjectAssignmentPatternMember::MObjectAssignmentPatternRest(node) => {
                node.format().fmt(f)
            }
            AnyMObjectAssignmentPatternMember::MObjectAssignmentPatternShorthandProperty(node) => {
                node.format().fmt(f)
            }
        }
    }
}
