use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use biome_formatter::write;
use sql_syntax::PsqlPlIfStatement;
use sql_syntax::PsqlPlIfStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlIfStatement;
impl FormatNodeRule<PsqlPlIfStatement> for FormatPsqlPlIfStatement {
    fn fmt_fields(&self, node: &PsqlPlIfStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlIfStatementFields {
            if_token,
            condition,
            then_token,
            statements,
            elsif_clauses,
            else_clause,
            end_token,
            end_if_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                if_token.format(),
                space(),
                condition.format(),
                space(),
                then_token.format(),
                FormatPlBranchStatements::new(&statements),
                elsif_clauses.format(),
            ]
        )?;
        if let Some(else_clause) = else_clause {
            write!(f, [else_clause.format()])?;
        }
        write!(
            f,
            [
                end_token.format(),
                space(),
                end_if_token.format(),
                semicolon_token.format()
            ]
        )
    }
}
