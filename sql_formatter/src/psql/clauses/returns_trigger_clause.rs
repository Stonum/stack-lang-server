use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlReturnsTriggerClause;
use sql_syntax::PsqlReturnsTriggerClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsTriggerClause;
impl FormatNodeRule<PsqlReturnsTriggerClause> for FormatPsqlReturnsTriggerClause {
    fn fmt_fields(
        &self,
        node: &PsqlReturnsTriggerClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlReturnsTriggerClauseFields { trigger_token } = node.as_fields();

        write!(f, [trigger_token.format()])
    }
}
