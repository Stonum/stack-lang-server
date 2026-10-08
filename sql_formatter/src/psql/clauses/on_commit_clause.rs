use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlOnCommitClause;
use sql_syntax::PsqlOnCommitClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOnCommitClause;
impl FormatNodeRule<PsqlOnCommitClause> for FormatPsqlOnCommitClause {
    fn fmt_fields(&self, node: &PsqlOnCommitClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlOnCommitClauseFields {
            on_token,
            commit_token,
            action,
            rows_token,
        } = node.as_fields();

        write!(
            f,
            [
                on_token.format(),
                space(),
                commit_token.format(),
                space(),
                action.format()
            ]
        )?;
        if let Some(rows_token) = rows_token {
            write!(f, [space(), rows_token.format()])?;
        }
        Ok(())
    }
}
