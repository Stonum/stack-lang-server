use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::PsqlXmlExistsExpression;
use sql_syntax::PsqlXmlExistsExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlExistsExpression;
impl FormatNodeRule<PsqlXmlExistsExpression> for FormatPsqlXmlExistsExpression {
    fn fmt_fields(&self, node: &PsqlXmlExistsExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlExistsExpressionFields {
            name_token,
            l_paren_token,
            xpath,
            passing,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                l_paren_token.format(),
                group(&soft_block_indent(&format_args![
                    xpath.format(),
                    soft_line_break_or_space(),
                    passing.format()
                ])),
                r_paren_token.format()
            ]
        )
    }
}
