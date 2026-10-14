use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::SqlJoinClause;
use sql_syntax::SqlJoinClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlJoinClause;
impl FormatNodeRule<SqlJoinClause> for FormatSqlJoinClause {
    fn fmt_fields(&self, node: &SqlJoinClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlJoinClauseFields {
            join_type,
            outer_token,
            join_token,
            source,
            nested_joins,
            on_token,
            condition,
            using_clause,
        } = node.as_fields();

        if let Some(join_type) = join_type {
            write!(f, [join_type.format(), space()])?;
        }
        if let Some(outer_token) = outer_token {
            write!(f, [outer_token.format(), space()])?;
        }
        write!(f, [join_token.format(), space(), source.format()])?;

        let constraint = format_with(|f| {
            if let Some(on_token) = &on_token {
                write!(f, [on_token.format()])?;
            }
            if let Some(condition) = &condition {
                write!(f, [space(), condition.format()])?;
            }
            if let Some(using_clause) = &using_clause {
                write!(f, [using_clause.format()])?;
            }
            Ok(())
        });

        // Nested joins and the outer join's own `ON` go on their own lines.
        if !nested_joins.is_empty() {
            return write!(
                f,
                [indent(&format_args![
                    nested_joins.format(),
                    hard_line_break(),
                    constraint
                ])]
            );
        }
        if on_token.is_some() || using_clause.is_some() {
            write!(f, [space(), constraint])?;
        }
        Ok(())
    }
}
