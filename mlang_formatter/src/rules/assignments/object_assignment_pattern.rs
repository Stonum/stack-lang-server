use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{MObjectAssignmentPattern, MObjectAssignmentPatternFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectAssignmentPattern;
impl_format_with_rule!(MObjectAssignmentPattern, FormatMObjectAssignmentPattern);

impl FormatNodeRule<MObjectAssignmentPattern> for FormatMObjectAssignmentPattern {
    fn fmt_fields(&self, node: &MObjectAssignmentPattern, f: &mut MFormatter) -> FormatResult<()> {
        let MObjectAssignmentPatternFields {
            at_token,
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        write!(f, [at_token.format(), l_curly_token.format()])?;

        if members.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            let should_insert_space_around_brackets = f.options().bracket_spacing().value();
            let should_expand = members.syntax().has_leading_newline();
            let members = members.format();

            let inner =
                &soft_block_indent_with_maybe_space(&members, should_insert_space_around_brackets);

            write!(f, [group(inner).should_expand(should_expand)])?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn needs_parentheses(&self, item: &MObjectAssignmentPattern) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &MObjectAssignmentPattern,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
