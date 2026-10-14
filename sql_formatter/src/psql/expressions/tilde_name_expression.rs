use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTildeNameExpression;
use sql_syntax::PsqlTildeNameExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeNameExpression;
impl FormatNodeRule<PsqlTildeNameExpression> for FormatPsqlTildeNameExpression {
    fn fmt_fields(&self, node: &PsqlTildeNameExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlTildeNameExpressionFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
