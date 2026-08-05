use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCteMaterializedHint;
use sql_syntax::SqlCteMaterializedHintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCteMaterializedHint;
impl FormatNodeRule<SqlCteMaterializedHint> for FormatSqlCteMaterializedHint {
    fn fmt_fields(&self, node: &SqlCteMaterializedHint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCteMaterializedHintFields {
            not_token,
            materialized_token,
        } = node.as_fields();

        if let Some(not_token) = not_token {
            write!(f, [not_token.format(), space()])?;
        }
        write!(f, [materialized_token.format()])
    }
}
