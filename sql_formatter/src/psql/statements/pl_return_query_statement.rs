use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlReturnQueryStatement;
use sql_syntax::PsqlPlReturnQueryStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlReturnQueryStatement;
impl FormatNodeRule<PsqlPlReturnQueryStatement> for FormatPsqlPlReturnQueryStatement {
    fn fmt_fields(
        &self,
        node: &PsqlPlReturnQueryStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPlReturnQueryStatementFields {
            return_token,
            query_token,
            query,
        } = node.as_fields();

        // A query that does not fit moves under `return query`, indented.
        write!(
            f,
            [
                return_token.format(),
                space(),
                query_token.format(),
                group(&soft_line_indent_or_space(&query.format()))
            ]
        )
    }
}
