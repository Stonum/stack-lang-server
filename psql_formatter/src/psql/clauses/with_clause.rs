use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlWithClause;
use psql_syntax::PsqlWithClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWithClause;
impl FormatNodeRule<PsqlWithClause> for FormatPsqlWithClause {
    fn fmt_fields(&self, node: &PsqlWithClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlWithClauseFields {
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
