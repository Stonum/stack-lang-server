use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlNullsOrder;
use sql_syntax::SqlNullsOrderFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlNullsOrder;
impl FormatNodeRule<SqlNullsOrder> for FormatSqlNullsOrder {
    fn fmt_fields(&self, node: &SqlNullsOrder, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlNullsOrderFields {
            nulls_token,
            position,
        } = node.as_fields();

        write!(f, [nulls_token.format(), space(), position.format()])
    }
}
