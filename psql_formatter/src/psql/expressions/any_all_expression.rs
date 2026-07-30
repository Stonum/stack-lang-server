use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlAnyAllExpression;
use psql_syntax::PsqlAnyAllExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAnyAllExpression;
impl FormatNodeRule<PsqlAnyAllExpression> for FormatPsqlAnyAllExpression {
    fn fmt_fields(&self, node: &PsqlAnyAllExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlAnyAllExpressionFields { quantifier, source } = node.as_fields();

        write!(f, [quantifier.format(), source.format()])
    }
}
