use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlFetchClause;
use psql_syntax::PsqlFetchClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFetchClause;
impl FormatNodeRule<PsqlFetchClause> for FormatPsqlFetchClause {
    fn fmt_fields(&self, node: &PsqlFetchClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFetchClauseFields {
            fetch_token,
            quantifier,
            count,
            row_or_rows,
            tail,
        } = node.as_fields();

        write!(f, [fetch_token.format(), space(), quantifier.format()])?;
        if let Some(count) = count {
            write!(f, [space(), count.format()])?;
        }
        write!(f, [space(), row_or_rows.format(), space(), tail.format(),])
    }
}
