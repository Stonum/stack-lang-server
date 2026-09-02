use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use biome_rowan::{AstNode, AstSeparatedList};
use mlang_syntax::{AnyMObjectAssignmentPatternMember, MObjectAssignmentPatternPropertyList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectAssignmentPatternPropertyList;
impl_format!(
    MObjectAssignmentPatternPropertyList,
    FormatMObjectAssignmentPatternPropertyList
);

impl FormatRule<MObjectAssignmentPatternPropertyList>
    for FormatMObjectAssignmentPatternPropertyList
{
    type Context = MFormatContext;

    fn fmt(
        &self,
        node: &MObjectAssignmentPatternPropertyList,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        // A trailing separator isn't allowed after a rest property.
        let has_trailing_rest = match node.into_iter().last() {
            Some(element) => matches!(
                element?,
                AnyMObjectAssignmentPatternMember::MObjectAssignmentPatternRest(_)
            ),
            None => false,
        };

        let trailing_separator = if has_trailing_rest {
            TrailingSeparator::Disallowed
        } else {
            FormatTrailingCommas::ES5.trailing_separator(f.options())
        };

        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(
            node.format_separated(",")
                .with_trailing_separator(trailing_separator),
        ) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
