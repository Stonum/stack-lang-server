use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlInsertValues;
use psql_syntax::PsqlInsertValuesFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInsertValues;
impl FormatNodeRule<PsqlInsertValues> for FormatPsqlInsertValues {
    fn fmt_fields(&self, node: &PsqlInsertValues, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlInsertValuesFields {
            values_token,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                values_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&items.format())),
                r_paren_token.format(),
            ]
        )
    }
}
