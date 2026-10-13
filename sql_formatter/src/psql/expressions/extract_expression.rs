use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlExtractExpression;
use sql_syntax::PsqlExtractExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlExtractExpression;
impl FormatNodeRule<PsqlExtractExpression> for FormatPsqlExtractExpression {
    fn fmt_fields(&self, node: &PsqlExtractExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlExtractExpressionFields {
            name_token,
            l_paren_token,
            field,
            from_token,
            source,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                l_paren_token.format(),
                field.format(),
                space(),
                from_token.format(),
                space(),
                source.format(),
                r_paren_token.format()
            ]
        )
    }
}
