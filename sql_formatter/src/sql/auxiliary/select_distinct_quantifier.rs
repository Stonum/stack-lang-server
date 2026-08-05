use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSelectDistinctQuantifier;
use sql_syntax::SqlSelectDistinctQuantifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSelectDistinctQuantifier;
impl FormatNodeRule<SqlSelectDistinctQuantifier> for FormatSqlSelectDistinctQuantifier {
    fn fmt_fields(
        &self,
        node: &SqlSelectDistinctQuantifier,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlSelectDistinctQuantifierFields {
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
