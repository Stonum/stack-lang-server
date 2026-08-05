use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlRoot;
use sql_syntax::SqlRootFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlRoot;
impl FormatNodeRule<SqlRoot> for FormatSqlRoot {
    fn fmt_fields(&self, node: &SqlRoot, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlRootFields {
            bom_token,
            stmt,
            eof_token,
        } = node.as_fields();

        // Preserved verbatim, not stripped -- some Windows-side tooling
        // that produced the original "UTF-8 with BOM" file may still rely
        // on it being there.
        if let Some(bom_token) = bom_token {
            write![f, [bom_token.format()]]?;
        }

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

    fn fmt_leading_comments(&self, _: &SqlRoot, _: &mut SqlFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, node: &SqlRoot, f: &mut SqlFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(node.syntax()),
            "Root should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &SqlRoot, _: &mut SqlFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
