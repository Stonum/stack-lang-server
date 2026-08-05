use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
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
            arguments,
            r_paren_token,
            filter_clause,
        } = node.as_fields();

        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])?;
        write_bracketed_fill_list(l_paren_token, &arguments, r_paren_token, f)?;
        if let Some(filter_clause) = filter_clause {
            write!(f, [space(), filter_clause.format()])?;
        }
        Ok(())
    }
}
