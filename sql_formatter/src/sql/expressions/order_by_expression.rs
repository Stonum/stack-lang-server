use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlOrderByExpression;
use sql_syntax::SqlOrderByExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlOrderByExpression;
impl FormatNodeRule<SqlOrderByExpression> for FormatSqlOrderByExpression {
    fn fmt_fields(&self, node: &SqlOrderByExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlOrderByExpressionFields { item, order } = node.as_fields();

        write!(f, [item.format()])?;
        if let Some(order) = order {
            write!(f, [space(), order.format()])?;
        }
        Ok(())
    }
}
