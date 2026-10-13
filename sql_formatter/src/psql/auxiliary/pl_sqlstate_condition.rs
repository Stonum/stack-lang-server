use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlSqlstateCondition;
use sql_syntax::PsqlPlSqlstateConditionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlSqlstateCondition;
impl FormatNodeRule<PsqlPlSqlstateCondition> for FormatPsqlPlSqlstateCondition {
    fn fmt_fields(&self, node: &PsqlPlSqlstateCondition, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlSqlstateConditionFields {
            sqlstate_token,
            code,
        } = node.as_fields();

        write!(f, [sqlstate_token.format(), space(), code.format()])
    }
}
