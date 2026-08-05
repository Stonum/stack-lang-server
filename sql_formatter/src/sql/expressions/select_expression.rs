use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSelectExpression;
use sql_syntax::SqlSelectExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSelectExpression;
impl FormatNodeRule<SqlSelectExpression> for FormatSqlSelectExpression {
    fn fmt_fields(&self, node: &SqlSelectExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSelectExpressionFields { expr, alias } = node.as_fields();

        write!(f, [expr.format()])?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
