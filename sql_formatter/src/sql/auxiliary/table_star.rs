use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTableStar;
use sql_syntax::SqlTableStarFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableStar;
impl FormatNodeRule<SqlTableStar> for FormatSqlTableStar {
    fn fmt_fields(&self, node: &SqlTableStar, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTableStarFields {
            table,
            dot_token,
            star,
        } = node.as_fields();

        write!(f, [table.format(), dot_token.format(), star.format()])
    }
}
