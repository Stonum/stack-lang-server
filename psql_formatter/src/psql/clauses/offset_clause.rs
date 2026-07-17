use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlOffsetClause;
use psql_syntax::PsqlOffsetClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOffsetClause;
impl FormatNodeRule<PsqlOffsetClause> for FormatPsqlOffsetClause {
    fn fmt_fields(&self, node: &PsqlOffsetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlOffsetClauseFields {
            offset_token,
            start,
        } = node.as_fields();

        write!(f, [offset_token.format(), space(), start.format()])
    }
}
