use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSubstringForClause;
use psql_syntax::PsqlSubstringForClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubstringForClause;
impl FormatNodeRule<PsqlSubstringForClause> for FormatPsqlSubstringForClause {
    fn fmt_fields(&self, node: &PsqlSubstringForClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSubstringForClauseFields { for_token, value } = node.as_fields();

        write!(f, [for_token.format(), space(), value.format()])
    }
}
