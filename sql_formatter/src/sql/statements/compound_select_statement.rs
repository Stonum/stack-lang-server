use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCompoundSelectStatement;
use sql_syntax::SqlCompoundSelectStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCompoundSelectStatement;
impl FormatNodeRule<SqlCompoundSelectStatement> for FormatSqlCompoundSelectStatement {
    fn fmt_fields(
        &self,
        node: &SqlCompoundSelectStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlCompoundSelectStatementFields {
            with_clause,
            first,
            set_operations,
            order_by_clause,
            limit_clause,
            offset_clause,
            fetch_clause,
            semicolon_token,
        } = node.as_fields();

        if let Some(with_clause) = with_clause {
            write!(f, [with_clause.format(), hard_line_break()])?;
        }

        // Same layout as `SqlSelectStatement`, with a parenthesized first operand.
        write!(
            f,
            [group(&format_once(|f| {
                write!(f, [first.format(), set_operations.format()])?;
                if let Some(order_by_clause) = order_by_clause {
                    write!(f, [soft_line_break_or_space(), order_by_clause.format()])?;
                }
                if let Some(limit_clause) = limit_clause {
                    write!(f, [soft_line_break_or_space(), limit_clause.format()])?;
                }
                if let Some(offset_clause) = offset_clause {
                    write!(f, [soft_line_break_or_space(), offset_clause.format()])?;
                }
                if let Some(fetch_clause) = fetch_clause {
                    write!(f, [soft_line_break_or_space(), fetch_clause.format()])?;
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
