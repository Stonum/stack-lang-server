use crate::prelude::*;
use crate::utils::write_select_body_clauses;
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

        // A simple statement ("select a from b") collapses onto one line if
        // it fits; anything that itself forces a hard break (a JOIN, a
        // wrapped list, an and/or chain, a subquery, ...) naturally expands
        // the whole group instead, one clause per line -- same
        // group-expansion propagation Points 4-8 already rely on.
        write!(
            f,
            [group(&format_once(|f| {
                write_select_body_clauses(
                    select_clause,
                    from_clause,
                    where_clause,
                    group_by_clause,
                    having_clause,
                    f,
                )?;
                write!(f, [set_operations.format()])?;
                if let Some(order_by_clause) = order_by_clause {
                    write!(f, [soft_line_break_or_space(), order_by_clause.format()])?;
                }
                if let Some(limit_clause) = limit_clause {
                    write!(f, [soft_line_break_or_space(), limit_clause.format()])?;
                }
                if let Some(offset_clause) = offset_clause {
                    write!(f, [soft_line_break_or_space(), offset_clause.format()])?;
                }
                Ok(())
            }))]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
