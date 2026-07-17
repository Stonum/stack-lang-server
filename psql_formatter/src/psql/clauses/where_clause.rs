use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlWhereClause;
use psql_syntax::PsqlWhereClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWhereClause;
impl FormatNodeRule<PsqlWhereClause> for FormatPsqlWhereClause {
    fn fmt_fields(&self, node: &PsqlWhereClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlWhereClauseFields {
            where_token,
            condition,
        } = node.as_fields();

        write!(f, [where_token.format(), space(), condition.format()])
    }
}
