use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlRoot;
use psql_syntax::PsqlRootFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlRoot;
impl FormatNodeRule<PsqlRoot> for FormatPsqlRoot {
    fn fmt_fields(&self, node: &PsqlRoot, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlRootFields { stmt, eof_token } = node.as_fields();

        write![f, [format_leading_comments(node.syntax())]]?;

        write![
            f,
            [
                stmt.format(),
                format_trailing_comments(node.syntax()),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        ]
    }

    fn fmt_leading_comments(&self, _: &PsqlRoot, _: &mut PsqlFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, node: &PsqlRoot, f: &mut PsqlFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(node.syntax()),
            "Root should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &PsqlRoot, _: &mut PsqlFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
