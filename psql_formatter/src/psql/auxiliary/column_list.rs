use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlColumnList;
use psql_syntax::PsqlColumnListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColumnList;
impl FormatNodeRule<PsqlColumnList> for FormatPsqlColumnList {
    fn fmt_fields(&self, node: &PsqlColumnList, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlColumnListFields {
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
