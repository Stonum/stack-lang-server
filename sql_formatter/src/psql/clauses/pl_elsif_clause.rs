use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use biome_formatter::write;
use sql_syntax::PsqlPlElsifClause;
use sql_syntax::PsqlPlElsifClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlElsifClause;
impl FormatNodeRule<PsqlPlElsifClause> for FormatPsqlPlElsifClause {
    fn fmt_fields(&self, node: &PsqlPlElsifClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlElsifClauseFields {
            elsif_token,
            condition,
            then_token,
            statements,
        } = node.as_fields();

        write!(
            f,
            [
                elsif_token.format(),
                space(),
                condition.format(),
                space(),
                then_token.format(),
                FormatPlBranchStatements::new(&statements),
            ]
        )
    }
}
