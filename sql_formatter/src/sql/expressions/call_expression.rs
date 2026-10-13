use crate::prelude::*;
use crate::utils::{is_simple_call_argument, write_bracketed_fill_list_with_affixes};
use biome_formatter::write;
use sql_syntax::AnySqlCallArgument;
use sql_syntax::SqlCallExpression;
use sql_syntax::SqlCallExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCallExpression;
impl FormatNodeRule<SqlCallExpression> for FormatSqlCallExpression {
    fn fmt_fields(&self, node: &SqlCallExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCallExpressionFields {
            schema,
            name,
            l_paren_token,
            quantifier,
            arguments,
            order_by,
            r_paren_token,
            filter_clause,
        } = node.as_fields();

        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])?;
        write_bracketed_fill_list_with_affixes(
            l_paren_token,
            quantifier.as_ref().map(|quantifier| quantifier.format()),
            &arguments,
            order_by.as_ref().map(|order_by| order_by.format()),
            r_paren_token,
            |argument: &AnySqlCallArgument| !is_simple_call_argument(argument, 0),
            f,
        )?;
        if let Some(filter_clause) = filter_clause {
            write!(f, [space(), filter_clause.format()])?;
        }
        Ok(())
    }
}
