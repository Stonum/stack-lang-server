use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlWithDataClause;
use sql_syntax::PsqlWithDataClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWithDataClause;
impl FormatNodeRule<PsqlWithDataClause> for FormatPsqlWithDataClause {
    fn fmt_fields(&self, node: &PsqlWithDataClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlWithDataClauseFields {
            with_token,
            no_token,
            data_token,
        } = node.as_fields();

        write!(f, [with_token.format()])?;
        if let Some(no_token) = no_token {
            write!(f, [space(), no_token.format()])?;
        }
        write!(f, [space(), data_token.format()])
    }
}
