use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlLimitClause;
use psql_syntax::PsqlLimitClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLimitClause;
impl FormatNodeRule<PsqlLimitClause> for FormatPsqlLimitClause {
    fn fmt_fields(&self, node: &PsqlLimitClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlLimitClauseFields {
            limit_token,
            limit_count,
        } = node.as_fields();

        write!(f, [limit_token.format(), space(), limit_count.format()])
    }
}
