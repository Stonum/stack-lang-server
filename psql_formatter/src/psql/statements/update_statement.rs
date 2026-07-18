use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlUpdateStatement;
use psql_syntax::PsqlUpdateStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlUpdateStatement;
impl FormatNodeRule<PsqlUpdateStatement> for FormatPsqlUpdateStatement {
    fn fmt_fields(&self, node: &PsqlUpdateStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlUpdateStatementFields {
            with_clause,
            update_token,
            table,
            set_clause,
            where_clause,
            returning_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        write!(f, [update_token.format(), space(), table.format()])?;
        write!(f, [hard_line_break(), set_clause.format()])?;

        if let Some(where_clause) = where_clause {
            write!(f, [hard_line_break(), where_clause.format()])?;
        }
        if let Some(returning_clause) = returning_clause {
            write!(f, [hard_line_break(), returning_clause.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
