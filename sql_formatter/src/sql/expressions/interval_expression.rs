use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlIntervalExpression;
use sql_syntax::SqlIntervalExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlIntervalExpression;
impl FormatNodeRule<SqlIntervalExpression> for FormatSqlIntervalExpression {
    fn fmt_fields(&self, node: &SqlIntervalExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlIntervalExpressionFields {
            interval_token,
            value,
        } = node.as_fields();

        write!(f, [interval_token.format(), space(), value.format()])
    }
}
