use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlFetchWithTiesTail;
use sql_syntax::SqlFetchWithTiesTailFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFetchWithTiesTail;
impl FormatNodeRule<SqlFetchWithTiesTail> for FormatSqlFetchWithTiesTail {
    fn fmt_fields(&self, node: &SqlFetchWithTiesTail, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFetchWithTiesTailFields {
            with_token,
            ties_token,
        } = node.as_fields();

        write!(f, [with_token.format(), space(), ties_token.format()])
    }
}
