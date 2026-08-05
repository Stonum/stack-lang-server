use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAnyAllExpression;
use sql_syntax::SqlAnyAllExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAnyAllExpression;
impl FormatNodeRule<SqlAnyAllExpression> for FormatSqlAnyAllExpression {
    fn fmt_fields(&self, node: &SqlAnyAllExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAnyAllExpressionFields { quantifier, source } = node.as_fields();

        write!(f, [quantifier.format(), source.format()])
    }
}
