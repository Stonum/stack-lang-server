use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSelectDistinctQuantifier;
use psql_syntax::PsqlSelectDistinctQuantifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectDistinctQuantifier;
impl FormatNodeRule<PsqlSelectDistinctQuantifier> for FormatPsqlSelectDistinctQuantifier {
    fn fmt_fields(
        &self,
        node: &PsqlSelectDistinctQuantifier,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlSelectDistinctQuantifierFields {
            distinct_token,
            on_clause,
        } = node.as_fields();

        write!(f, [distinct_token.format()])?;
        if let Some(on_clause) = on_clause {
            write!(f, [space(), on_clause.format()])?;
        }
        Ok(())
    }
}
