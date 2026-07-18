use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlArrayExpression;
use psql_syntax::PsqlArrayExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlArrayExpression;
impl FormatNodeRule<PsqlArrayExpression> for FormatPsqlArrayExpression {
    fn fmt_fields(&self, node: &PsqlArrayExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlArrayExpressionFields {
            array_token,
            l_brack_token,
            items,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                array_token.format(),
                l_brack_token.format(),
                group(&soft_block_indent(&items.format())),
                r_brack_token.format(),
            ]
        )
    }
}
