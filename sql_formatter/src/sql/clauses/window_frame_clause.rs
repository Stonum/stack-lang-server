use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWindowFrameClause;
use sql_syntax::SqlWindowFrameClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowFrameClause;
impl FormatNodeRule<SqlWindowFrameClause> for FormatSqlWindowFrameClause {
    fn fmt_fields(&self, node: &SqlWindowFrameClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlWindowFrameClauseFields {
            unit,
            between_token,
            start,
            and_token,
            end,
        } = node.as_fields();

        write!(f, [unit.format()])?;
        if let Some(between_token) = between_token {
            write!(f, [space(), between_token.format()])?;
        }
        write!(f, [space(), start.format()])?;
        if let Some(and_token) = and_token {
            write!(f, [space(), and_token.format()])?;
        }
        if let Some(end) = end {
            write!(f, [space(), end.format()])?;
        }
        Ok(())
    }
}
