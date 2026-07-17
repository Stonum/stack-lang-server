use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSelectExpression;
use psql_syntax::PsqlSelectExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectExpression;
impl FormatNodeRule<PsqlSelectExpression> for FormatPsqlSelectExpression {
    fn fmt_fields(&self, node: &PsqlSelectExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSelectExpressionFields { expr, alias } = node.as_fields();

        write!(f, [expr.format()])?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
