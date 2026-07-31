use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlExistsExpression;
use psql_syntax::PsqlExistsExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlExistsExpression;
impl FormatNodeRule<PsqlExistsExpression> for FormatPsqlExistsExpression {
    fn fmt_fields(&self, node: &PsqlExistsExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlExistsExpressionFields {
            exists_token,
            subquery,
        } = node.as_fields();

        write!(f, [exists_token.format(), space(), subquery.format()])
    }
}
