use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlAliasColumnList;
use psql_syntax::PsqlAliasColumnListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAliasColumnList;
impl FormatNodeRule<PsqlAliasColumnList> for FormatPsqlAliasColumnList {
    fn fmt_fields(&self, node: &PsqlAliasColumnList, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlAliasColumnListFields {
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
