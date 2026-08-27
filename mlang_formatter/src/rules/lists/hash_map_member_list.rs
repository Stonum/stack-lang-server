use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use crate::utils::member_chain::is_simple_object_member;
use crate::utils::{COMPACT_FILL_THRESHOLD, write_compact_fill, write_with_custom_line_width};
use biome_rowan::{AstNode, AstSeparatedList};
use mlang_syntax::MHashMapMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMHashMapMemberList;
impl_format!(MHashMapMemberList, FormatMHashMapMemberList);

impl FormatRule<MHashMapMemberList> for FormatMHashMapMemberList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MHashMapMemberList, f: &mut MFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

        if node.len() >= COMPACT_FILL_THRESHOLD {
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
