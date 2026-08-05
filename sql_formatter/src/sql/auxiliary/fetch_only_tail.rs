use crate::prelude::*;
use sql_syntax::SqlFetchOnlyTail;
use sql_syntax::SqlFetchOnlyTailFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFetchOnlyTail;
impl FormatNodeRule<SqlFetchOnlyTail> for FormatSqlFetchOnlyTail {
    fn fmt_fields(&self, node: &SqlFetchOnlyTail, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFetchOnlyTailFields { only_token } = node.as_fields();

        only_token.format().fmt(f)
    }
}
