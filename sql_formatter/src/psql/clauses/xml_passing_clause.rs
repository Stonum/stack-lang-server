use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlPassingClause;
use sql_syntax::PsqlXmlPassingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlPassingClause;
impl FormatNodeRule<PsqlXmlPassingClause> for FormatPsqlXmlPassingClause {
    fn fmt_fields(&self, node: &PsqlXmlPassingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlPassingClauseFields {
            passing_token,
            leading_by,
            value,
            trailing_by,
        } = node.as_fields();

        write!(f, [passing_token.format()])?;
        if let Some(leading_by) = leading_by {
            write!(f, [space(), leading_by.format()])?;
        }
        write!(f, [space(), value.format()])?;
        if let Some(trailing_by) = trailing_by {
            write!(f, [space(), trailing_by.format()])?;
        }
        Ok(())
    }
}
