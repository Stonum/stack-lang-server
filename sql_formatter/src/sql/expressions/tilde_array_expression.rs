use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use sql_syntax::SqlTildeArrayExpression;
use sql_syntax::SqlTildeArrayExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTildeArrayExpression;
impl FormatNodeRule<SqlTildeArrayExpression> for FormatSqlTildeArrayExpression {
    fn fmt_fields(&self, node: &SqlTildeArrayExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTildeArrayExpressionFields {
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
