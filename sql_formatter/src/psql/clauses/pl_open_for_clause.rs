use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlOpenForClause;
use sql_syntax::PsqlPlOpenForClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlOpenForClause;
impl FormatNodeRule<PsqlPlOpenForClause> for FormatPsqlPlOpenForClause {
    fn fmt_fields(&self, node: &PsqlPlOpenForClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlOpenForClauseFields { for_token, source } = node.as_fields();

        // A query that does not fit moves under `open ... for`, indented.
        write!(
            f,
            [
                for_token.format(),
                group(&soft_line_indent_or_space(&source.format()))
            ]
        )
    }
}
