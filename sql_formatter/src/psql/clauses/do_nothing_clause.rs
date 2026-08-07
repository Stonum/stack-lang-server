use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlDoNothingClause;
use sql_syntax::PsqlDoNothingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDoNothingClause;
impl FormatNodeRule<PsqlDoNothingClause> for FormatPsqlDoNothingClause {
    fn fmt_fields(&self, node: &PsqlDoNothingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlDoNothingClauseFields {
            do_token,
            nothing_token,
        } = node.as_fields();

        write!(f, [do_token.format(), space(), nothing_token.format()])
    }
}
