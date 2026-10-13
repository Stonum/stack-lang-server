use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use sql_syntax::AnySqlSelectItem;
use sql_syntax::PsqlPlPerformStatement;
use sql_syntax::PsqlPlPerformStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlPerformStatement;
impl FormatNodeRule<PsqlPlPerformStatement> for FormatPsqlPlPerformStatement {
    fn fmt_fields(&self, node: &PsqlPlPerformStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlPerformStatementFields {
            perform_token,
            list,
            from_clause,
            where_clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_once(|f| {
                // Same list layout as `select` (`select_clause.rs`).
                write_wrapping_fill_clause(
                    perform_token.format(),
                    &list,
                    |item: &AnySqlSelectItem| match item {
                        AnySqlSelectItem::SqlSelectExpression(item) => item
                            .expr()
                            .is_ok_and(|expr| !is_simple_expression(&expr, 0)),
                        AnySqlSelectItem::SqlStar(_) => false,
                        AnySqlSelectItem::SqlTableStar(_) => false,
                    },
                    f,
                )?;
                if let Some(from_clause) = from_clause {
                    write!(f, [soft_line_break_or_space(), from_clause.format()])?;
                }
                if let Some(where_clause) = where_clause {
                    write!(f, [soft_line_break_or_space(), where_clause.format()])?;
                }
                Ok(())
            }))]
        )?;
        write!(f, [semicolon_token.format()])
    }
}
