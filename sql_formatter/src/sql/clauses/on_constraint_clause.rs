use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlOnConstraintClause;
use sql_syntax::SqlOnConstraintClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlOnConstraintClause;
impl FormatNodeRule<SqlOnConstraintClause> for FormatSqlOnConstraintClause {
    fn fmt_fields(&self, node: &SqlOnConstraintClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlOnConstraintClauseFields {
            on_token,
            constraint_token,
            name,
        } = node.as_fields();

        write!(
            f,
            [
                on_token.format(),
                space(),
                constraint_token.format(),
                space(),
                name.format(),
            ]
        )
    }
}
