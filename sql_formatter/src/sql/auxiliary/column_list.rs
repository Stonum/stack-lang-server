use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlColumnList;
use sql_syntax::SqlColumnListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnList;
impl FormatNodeRule<SqlColumnList> for FormatSqlColumnList {
    fn fmt_fields(&self, node: &SqlColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlColumnListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&items.format())),
                r_paren_token.format(),
            ]
        )
    }
}
