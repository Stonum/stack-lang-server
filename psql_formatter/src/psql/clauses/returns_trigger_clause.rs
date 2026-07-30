use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlReturnsTriggerClause;
use psql_syntax::PsqlReturnsTriggerClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsTriggerClause;
impl FormatNodeRule<PsqlReturnsTriggerClause> for FormatPsqlReturnsTriggerClause {
    fn fmt_fields(
        &self,
        node: &PsqlReturnsTriggerClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlReturnsTriggerClauseFields { trigger_token } = node.as_fields();

        write!(f, [trigger_token.format()])
    }
}
