use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSelectStatement;
use psql_syntax::PsqlSelectStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectStatement;
impl FormatNodeRule<PsqlSelectStatement> for FormatPsqlSelectStatement {
    fn fmt_fields(&self, node: &PsqlSelectStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSelectStatementFields {
            with_clause,
            select_clause,
            from_clause,
            where_clause,
            group_by_clause,
            having_clause,
            set_operations,
            order_by_clause,
            limit_clause,
            offset_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        write!(f, [select_clause.format()])?;

        if let Some(from_clause) = from_clause {
            write!(f, [hard_line_break(), from_clause.format()])?;
        }
        if let Some(where_clause) = where_clause {
            write!(f, [hard_line_break(), where_clause.format()])?;
        }
        if let Some(group_by_clause) = group_by_clause {
            write!(f, [hard_line_break(), group_by_clause.format()])?;
        }
        if let Some(having_clause) = having_clause {
            write!(f, [hard_line_break(), having_clause.format()])?;
        }

        write!(f, [set_operations.format()])?;

        if let Some(order_by_clause) = order_by_clause {
            write!(f, [hard_line_break(), order_by_clause.format()])?;
        }
        if let Some(limit_clause) = limit_clause {
            write!(f, [hard_line_break(), limit_clause.format()])?;
        }
        if let Some(offset_clause) = offset_clause {
            write!(f, [hard_line_break(), offset_clause.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
