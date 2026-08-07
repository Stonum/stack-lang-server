use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlReturnsSetofClause;
use sql_syntax::PsqlReturnsSetofClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsSetofClause;
impl FormatNodeRule<PsqlReturnsSetofClause> for FormatPsqlReturnsSetofClause {
    fn fmt_fields(&self, node: &PsqlReturnsSetofClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlReturnsSetofClauseFields { setof_token, ty } = node.as_fields();

        write!(f, [setof_token.format(), space(), ty.format()])
    }
}
