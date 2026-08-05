use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlOffsetClause;
use sql_syntax::SqlOffsetClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlOffsetClause;
impl FormatNodeRule<SqlOffsetClause> for FormatSqlOffsetClause {
    fn fmt_fields(&self, node: &SqlOffsetClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlOffsetClauseFields {
            offset_token,
            start,
        } = node.as_fields();

        write!(f, [offset_token.format(), space(), start.format()])
    }
}
