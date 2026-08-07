use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTriggerForEachClause;
use sql_syntax::PsqlTriggerForEachClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerForEachClause;
impl FormatNodeRule<PsqlTriggerForEachClause> for FormatPsqlTriggerForEachClause {
    fn fmt_fields(
        &self,
        node: &PsqlTriggerForEachClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTriggerForEachClauseFields {
            for_token,
            each_token,
            granularity,
        } = node.as_fields();

        write!(
            f,
            [
                for_token.format(),
                space(),
                each_token.format(),
                space(),
                granularity.format()
            ]
        )
    }
}
