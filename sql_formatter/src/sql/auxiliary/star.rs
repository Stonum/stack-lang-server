use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlStar;
use sql_syntax::SqlStarFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlStar;
impl FormatNodeRule<SqlStar> for FormatSqlStar {
    fn fmt_fields(&self, node: &SqlStar, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlStarFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
