use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlOnConflictClause;
use psql_syntax::PsqlOnConflictClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOnConflictClause;
impl FormatNodeRule<PsqlOnConflictClause> for FormatPsqlOnConflictClause {
    fn fmt_fields(&self, node: &PsqlOnConflictClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlOnConflictClauseFields {
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
