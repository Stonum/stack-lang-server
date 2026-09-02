use crate::prelude::*;
use mlang_syntax::AnyMArrayAssignmentPatternElement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMArrayAssignmentPatternElement;
impl_format!(
    AnyMArrayAssignmentPatternElement,
    FormatAnyMArrayAssignmentPatternElement
);

impl FormatRule<AnyMArrayAssignmentPatternElement> for FormatAnyMArrayAssignmentPatternElement {
    type Context = MFormatContext;
    fn fmt(
        &self,
        node: &AnyMArrayAssignmentPatternElement,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyMArrayAssignmentPatternElement::AnyMAssignment(node) => node.format().fmt(f),
            AnyMArrayAssignmentPatternElement::MArrayAssignmentPatternRestElement(node) => {
                node.format().fmt(f)
            }
            AnyMArrayAssignmentPatternElement::MArrayHole(node) => node.format().fmt(f),
        }
    }
}
