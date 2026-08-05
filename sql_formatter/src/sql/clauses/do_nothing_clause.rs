use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDoNothingClause;
use sql_syntax::SqlDoNothingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDoNothingClause;
impl FormatNodeRule<SqlDoNothingClause> for FormatSqlDoNothingClause {
    fn fmt_fields(&self, node: &SqlDoNothingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDoNothingClauseFields {
            do_token,
            nothing_token,
        } = node.as_fields();

        write!(f, [do_token.format(), space(), nothing_token.format()])
    }
}
