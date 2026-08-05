use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use psql_syntax::PsqlValuesRow;
use psql_syntax::PsqlValuesRowFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlValuesRow;
impl FormatNodeRule<PsqlValuesRow> for FormatPsqlValuesRow {
    fn fmt_fields(&self, node: &PsqlValuesRow, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlValuesRowFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, f)
    }
}
