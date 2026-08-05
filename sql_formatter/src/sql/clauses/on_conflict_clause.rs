use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlOnConflictClause;
use sql_syntax::SqlOnConflictClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlOnConflictClause;
impl FormatNodeRule<SqlOnConflictClause> for FormatSqlOnConflictClause {
    fn fmt_fields(&self, node: &SqlOnConflictClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlOnConflictClauseFields {
            on_token,
            conflict_token,
            target,
            action,
        } = node.as_fields();

        write!(f, [on_token.format(), space(), conflict_token.format()])?;
        if let Some(target) = target {
            write!(f, [space(), target.format()])?;
        }
        write!(f, [space(), action.format()])
    }
}
