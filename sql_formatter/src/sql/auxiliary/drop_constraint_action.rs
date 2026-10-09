use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropConstraintAction;
use sql_syntax::SqlDropConstraintActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropConstraintAction;
impl FormatNodeRule<SqlDropConstraintAction> for FormatSqlDropConstraintAction {
    fn fmt_fields(&self, node: &SqlDropConstraintAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropConstraintActionFields {
            drop_token,
            constraint_token,
            if_token,
            exists_token,
            name,
            drop_behavior,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), constraint_token.format()])?;
        for token in [&if_token, &exists_token].into_iter().flatten() {
            write!(f, [space(), token.format()])?;
        }
        write!(f, [space(), name.format()])?;
        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        Ok(())
    }
}
