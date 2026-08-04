use crate::prelude::*;
use psql_syntax::PsqlFetchOnlyTail;
use psql_syntax::PsqlFetchOnlyTailFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFetchOnlyTail;
impl FormatNodeRule<PsqlFetchOnlyTail> for FormatPsqlFetchOnlyTail {
    fn fmt_fields(&self, node: &PsqlFetchOnlyTail, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFetchOnlyTailFields { only_token } = node.as_fields();

        only_token.format().fmt(f)
    }
}
