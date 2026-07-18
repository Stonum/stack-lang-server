use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDeleteStatement;
use psql_syntax::PsqlDeleteStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDeleteStatement;
impl FormatNodeRule<PsqlDeleteStatement> for FormatPsqlDeleteStatement {
    fn fmt_fields(&self, node: &PsqlDeleteStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlDeleteStatementFields {
            with_clause,
            delete_token,
            from_token,
            table,
            using,
            where_clause,
            returning_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        write!(
            f,
            [
                delete_token.format(),
                space(),
                from_token.format(),
                space(),
                table.format()
            ]
        )?;

        if let Some(using) = using {
            write!(f, [hard_line_break(), using.format()])?;
        }
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
