use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAddColumnAction;
use sql_syntax::SqlAddColumnActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAddColumnAction;
impl FormatNodeRule<SqlAddColumnAction> for FormatSqlAddColumnAction {
    fn fmt_fields(&self, node: &SqlAddColumnAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAddColumnActionFields {
            add_token,
            column_token,
            if_token,
            not_token,
            exists_token,
            column,
        } = node.as_fields();

        write!(f, [add_token.format()])?;
        for token in [&column_token, &if_token, &not_token, &exists_token]
            .into_iter()
            .flatten()
        {
            write!(f, [space(), token.format()])?;
        }
        write!(f, [space(), column.format()])
    }
}
