use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{MArrayAssignmentPattern, MArrayAssignmentPatternFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMArrayAssignmentPattern;
impl_format_with_rule!(MArrayAssignmentPattern, FormatMArrayAssignmentPattern);

impl FormatNodeRule<MArrayAssignmentPattern> for FormatMArrayAssignmentPattern {
    fn fmt_fields(&self, node: &MArrayAssignmentPattern, f: &mut MFormatter) -> FormatResult<()> {
        let MArrayAssignmentPatternFields {
            at_token,
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(f, [at_token.format(), l_brack_token.format()])?;

        if elements.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            write!(f, [group(&soft_block_indent(&elements.format()))])?;
        }

        write!(f, [r_brack_token.format()])
    }

    fn needs_parentheses(&self, item: &MArrayAssignmentPattern) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &MArrayAssignmentPattern,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
