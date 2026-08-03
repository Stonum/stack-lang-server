use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use psql_syntax::AnyPsqlSelectItem;
use psql_syntax::PsqlSelectClause;
use psql_syntax::PsqlSelectClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectClause;
impl FormatNodeRule<PsqlSelectClause> for FormatPsqlSelectClause {
    fn fmt_fields(&self, node: &PsqlSelectClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSelectClauseFields {
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
            |item: &AnyPsqlSelectItem| match item {
                AnyPsqlSelectItem::PsqlSelectExpression(item) => item
                    .expr()
                    .is_ok_and(|expr| !is_simple_expression(&expr, 0)),
                AnyPsqlSelectItem::PsqlStar(_) => false,
                AnyPsqlSelectItem::PsqlTableStar(_) => false,
            },
            f,
        )
    }
}
