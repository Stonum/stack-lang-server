use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlOnConstraintClause;
use psql_syntax::PsqlOnConstraintClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOnConstraintClause;
impl FormatNodeRule<PsqlOnConstraintClause> for FormatPsqlOnConstraintClause {
    fn fmt_fields(&self, node: &PsqlOnConstraintClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlOnConstraintClauseFields {
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
