use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTrimExpression;
use sql_syntax::PsqlTrimExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTrimExpression;
impl FormatNodeRule<PsqlTrimExpression> for FormatPsqlTrimExpression {
    fn fmt_fields(&self, node: &PsqlTrimExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlTrimExpressionFields {
            name_token,
            l_paren_token,
            side,
            characters,
            from_token,
            string,
            r_paren_token,
        } = node.as_fields();

        write!(f, [name_token.format(), l_paren_token.format()])?;
        if let Some(side) = side {
            write!(f, [side.format(), space()])?;
        }
        if let Some(characters) = characters {
            write!(f, [characters.format(), space()])?;
        }
        write!(
            f,
            [
                from_token.format(),
                space(),
                string.format(),
                r_paren_token.format()
            ]
        )
    }
}
