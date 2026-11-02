use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use crate::utils::member_chain::is_simple_object_member;
use crate::utils::{
    COMPACT_FILL_THRESHOLD, contains_multi_line_string_token, write_compact_fill,
    write_with_custom_line_width,
};
use biome_rowan::{AstNode, AstSeparatedList};
use mlang_syntax::MObjectMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectMemberList;
impl_format!(MObjectMemberList, FormatMObjectMemberList);

impl FormatRule<MObjectMemberList> for FormatMObjectMemberList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MObjectMemberList, f: &mut MFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

        if node.len() >= COMPACT_FILL_THRESHOLD && !has_multi_line_string_member(node) {
            // Like call arguments, the compact fill layout packs entries
            // against the narrower `pretty_line_width`, not the full line
            // width.
            let custom_width = f.options().pretty_line_width();

            return write_with_custom_line_width(
                f,
                custom_width,
                node.syntax(),
                format_with(|f| {
                    let entries = node
                        .elements()
                        .zip(
                            node.format_separated(",")
                                .with_trailing_separator(trailing_separator),
                        )
                        .map(|(element, formatted)| {
                            let (is_simple, leading_lines) = match element.node() {
                                Ok(node) => (
                                    is_simple_object_member(node),
                                    get_lines_before(node.syntax()),
                                ),
                                Err(_) => (false, 0),
                            };
                            (is_simple, leading_lines, formatted)
                        })
                        .collect();

                    write_compact_fill(f, entries)
                }),
            );
        }

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

/// Whether any member's subtree contains a multi-line string literal (including
/// embedded SQL). Such a member must not go through the compact-fill layout's
/// `write_with_custom_line_width`: that path re-splices the reformatted list
/// back into the real document line by line, baking the ambient indent *inside*
/// the string's own embedded newlines (see [contains_multi_line_string_token]).
/// Falling back to the ordinary join keeps each member on the non-splicing
/// formatting path, preserving the literal's interior indentation.
fn has_multi_line_string_member(list: &MObjectMemberList) -> bool {
    list.iter()
        .filter_map(|member| member.ok())
        .any(|member| contains_multi_line_string_token(member.syntax()))
}
