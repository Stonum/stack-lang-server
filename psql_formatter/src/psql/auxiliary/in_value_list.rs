use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use psql_syntax::PsqlInValueList;
use psql_syntax::PsqlInValueListFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlInValueList;
impl FormatNodeRule<PsqlInValueList> for FormatPsqlInValueList {
    fn fmt_fields(&self, node: &PsqlInValueList, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlInValueListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write_bracketed_fill_list(l_paren_token, &items, r_paren_token, f)
    }
}
