use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use sql_syntax::AnySqlSelectItem;
use sql_syntax::SqlSelectClause;
use sql_syntax::SqlSelectClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSelectClause;
impl FormatNodeRule<SqlSelectClause> for FormatSqlSelectClause {
    fn fmt_fields(&self, node: &SqlSelectClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSelectClauseFields {
            select_token,
            quantifier,
            list,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [select_token.format()])?;
            if let Some(quantifier) = &quantifier {
                write!(f, [space(), quantifier.format()])?;
            }
            Ok(())
        });

        write_wrapping_fill_clause(
            keyword,
            &list,
            |item: &AnySqlSelectItem| match item {
                AnySqlSelectItem::SqlSelectExpression(item) => item
                    .expr()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0)),
                AnySqlSelectItem::SqlStar(_) => false,
                AnySqlSelectItem::SqlTableStar(_) => false,
            },
            f,
        )
    }
}
