use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlSubstringFromClause;
use sql_syntax::PsqlSubstringFromClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubstringFromClause;
impl FormatNodeRule<PsqlSubstringFromClause> for FormatPsqlSubstringFromClause {
    fn fmt_fields(&self, node: &PsqlSubstringFromClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlSubstringFromClauseFields { from_token, value } = node.as_fields();

        write!(f, [from_token.format(), space(), value.format()])
    }
}
