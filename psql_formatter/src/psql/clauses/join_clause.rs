use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlJoinClause;
use psql_syntax::PsqlJoinClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlJoinClause;
impl FormatNodeRule<PsqlJoinClause> for FormatPsqlJoinClause {
    fn fmt_fields(&self, node: &PsqlJoinClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlJoinClauseFields {
            join_type,
            outer_token,
            join_token,
            source,
            on_token,
            condition,
        } = node.as_fields();

        if let Some(join_type) = join_type {
            write!(f, [join_type.format(), space()])?;
        }
        if let Some(outer_token) = outer_token {
            write!(f, [outer_token.format(), space()])?;
        }
        write!(f, [join_token.format(), space(), source.format()])?;

        if let Some(on_token) = on_token {
            write!(f, [space(), on_token.format()])?;
        }
        if let Some(condition) = condition {
            write!(f, [space(), condition.format()])?;
        }
        Ok(())
    }
}
