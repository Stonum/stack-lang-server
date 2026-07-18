use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCaseElseClause;
use psql_syntax::PsqlCaseElseClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCaseElseClause;
impl FormatNodeRule<PsqlCaseElseClause> for FormatPsqlCaseElseClause {
    fn fmt_fields(&self, node: &PsqlCaseElseClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlCaseElseClauseFields { else_token, result } = node.as_fields();

        write!(f, [else_token.format(), space(), result.format()])
    }
}
