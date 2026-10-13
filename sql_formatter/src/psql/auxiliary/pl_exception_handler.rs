use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use biome_formatter::write;
use sql_syntax::PsqlPlExceptionHandler;
use sql_syntax::PsqlPlExceptionHandlerFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExceptionHandler;
impl FormatNodeRule<PsqlPlExceptionHandler> for FormatPsqlPlExceptionHandler {
    fn fmt_fields(&self, node: &PsqlPlExceptionHandler, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlExceptionHandlerFields {
            when_token,
            conditions,
            then_token,
            statements,
        } = node.as_fields();

        write!(
            f,
            [
                when_token.format(),
                space(),
                conditions.format(),
                space(),
                then_token.format(),
                FormatPlBranchStatements::new(&statements)
            ]
        )
    }
}
