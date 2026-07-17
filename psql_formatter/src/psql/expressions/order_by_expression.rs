use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlOrderByExpression;
use psql_syntax::PsqlOrderByExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOrderByExpression;
impl FormatNodeRule<PsqlOrderByExpression> for FormatPsqlOrderByExpression {
    fn fmt_fields(&self, node: &PsqlOrderByExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlOrderByExpressionFields { item, order } = node.as_fields();

        write!(f, [item.format()])?;
        if let Some(order) = order {
            write!(f, [space(), order.format()])?;
        }
        Ok(())
    }
}
