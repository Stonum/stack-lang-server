use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAddConstraintAction;
use sql_syntax::SqlAddConstraintActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAddConstraintAction;
impl FormatNodeRule<SqlAddConstraintAction> for FormatSqlAddConstraintAction {
    fn fmt_fields(&self, node: &SqlAddConstraintAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAddConstraintActionFields {
            add_token,
            constraint,
        } = node.as_fields();

        write!(f, [add_token.format(), space(), constraint.format()])
    }
}
