use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAlterTriggerAction;
use sql_syntax::SqlAlterTriggerActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlterTriggerAction;
impl FormatNodeRule<SqlAlterTriggerAction> for FormatSqlAlterTriggerAction {
    fn fmt_fields(&self, node: &SqlAlterTriggerAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAlterTriggerActionFields {
            action,
            trigger_token,
            target,
        } = node.as_fields();

        write!(
            f,
            [
                action.format(),
                space(),
                trigger_token.format(),
                space(),
                target.format()
            ]
        )
    }
}
