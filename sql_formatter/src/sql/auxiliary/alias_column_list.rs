use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAliasColumnList;
use sql_syntax::SqlAliasColumnListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAliasColumnList;
impl FormatNodeRule<SqlAliasColumnList> for FormatSqlAliasColumnList {
    fn fmt_fields(&self, node: &SqlAliasColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAliasColumnListFields {
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
