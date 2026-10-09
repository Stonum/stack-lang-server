use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlIndexMethodClause;
use sql_syntax::PsqlIndexMethodClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlIndexMethodClause;
impl FormatNodeRule<PsqlIndexMethodClause> for FormatPsqlIndexMethodClause {
    fn fmt_fields(&self, node: &PsqlIndexMethodClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlIndexMethodClauseFields {
            using_token,
            method,
        } = node.as_fields();

        write!(f, [using_token.format(), space(), method.format()])
    }
}
