use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
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

        write!(f, [array_token.format()])?;
        write_bracketed_fill_list(l_brack_token, &items, r_brack_token, f)
    }
}
