use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlIntervalExpression;
use psql_syntax::PsqlIntervalExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlIntervalExpression;
impl FormatNodeRule<PsqlIntervalExpression> for FormatPsqlIntervalExpression {
    fn fmt_fields(&self, node: &PsqlIntervalExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlIntervalExpressionFields {
            interval_token,
            value,
        } = node.as_fields();

        write!(f, [interval_token.format(), space(), value.format()])
    }
}
