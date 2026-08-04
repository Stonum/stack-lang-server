use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlFetchWithTiesTail;
use psql_syntax::PsqlFetchWithTiesTailFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFetchWithTiesTail;
impl FormatNodeRule<PsqlFetchWithTiesTail> for FormatPsqlFetchWithTiesTail {
    fn fmt_fields(&self, node: &PsqlFetchWithTiesTail, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFetchWithTiesTailFields {
            with_token,
            ties_token,
        } = node.as_fields();

        write!(f, [with_token.format(), space(), ties_token.format()])
    }
}
