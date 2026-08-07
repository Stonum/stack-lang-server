use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlReturnsClause;
use sql_syntax::PsqlReturnsClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsClause;
impl FormatNodeRule<PsqlReturnsClause> for FormatPsqlReturnsClause {
    fn fmt_fields(&self, node: &PsqlReturnsClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlReturnsClauseFields { returns_token, ty } = node.as_fields();

        write!(f, [returns_token.format(), space(), ty.format()])
    }
}
