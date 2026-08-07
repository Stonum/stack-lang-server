use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCteMaterializedHint;
use sql_syntax::PsqlCteMaterializedHintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteMaterializedHint;
impl FormatNodeRule<PsqlCteMaterializedHint> for FormatPsqlCteMaterializedHint {
    fn fmt_fields(&self, node: &PsqlCteMaterializedHint, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCteMaterializedHintFields {
            not_token,
            materialized_token,
        } = node.as_fields();

        if let Some(not_token) = not_token {
            write!(f, [not_token.format(), space()])?;
        }
        write!(f, [materialized_token.format()])
    }
}
