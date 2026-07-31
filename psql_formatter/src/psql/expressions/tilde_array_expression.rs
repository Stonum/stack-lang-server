use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use psql_syntax::PsqlTildeArrayExpression;
use psql_syntax::PsqlTildeArrayExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeArrayExpression;
impl FormatNodeRule<PsqlTildeArrayExpression> for FormatPsqlTildeArrayExpression {
    fn fmt_fields(
        &self,
        node: &PsqlTildeArrayExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTildeArrayExpressionFields {
            array_token,
            open_tilde_token,
            l_brack_token,
            items,
            r_brack_token,
            close_tilde_token,
        } = node.as_fields();

        write!(f, [array_token.format(), open_tilde_token.format()])?;
        write_bracketed_fill_list(l_brack_token, &items, r_brack_token, f)?;
        write!(f, [close_tilde_token.format()])
    }
}
