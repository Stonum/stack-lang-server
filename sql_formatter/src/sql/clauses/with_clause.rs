use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWithClause;
use sql_syntax::SqlWithClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWithClause;
impl FormatNodeRule<SqlWithClause> for FormatSqlWithClause {
    fn fmt_fields(&self, node: &SqlWithClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlWithClauseFields {
            with_token,
            recursive_token,
            ctes,
        } = node.as_fields();

        write!(f, [with_token.format()])?;
        if let Some(recursive_token) = recursive_token {
            write!(f, [space(), recursive_token.format()])?;
        }
        write!(f, [space(), ctes.format()])
    }
}
