use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use biome_formatter::write;
use sql_syntax::PsqlPlElseClause;
use sql_syntax::PsqlPlElseClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlElseClause;
impl FormatNodeRule<PsqlPlElseClause> for FormatPsqlPlElseClause {
    fn fmt_fields(&self, node: &PsqlPlElseClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlElseClauseFields {
            else_token,
            statements,
        } = node.as_fields();

        write!(
            f,
            [
                else_token.format(),
                FormatPlBranchStatements::new(&statements)
            ]
        )
    }
}
