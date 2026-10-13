use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWindowFrameUnboundedBound;
use sql_syntax::SqlWindowFrameUnboundedBoundFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowFrameUnboundedBound;
impl FormatNodeRule<SqlWindowFrameUnboundedBound> for FormatSqlWindowFrameUnboundedBound {
    fn fmt_fields(
        &self,
        node: &SqlWindowFrameUnboundedBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlWindowFrameUnboundedBoundFields {
            unbounded_token,
            direction,
        } = node.as_fields();

        write!(f, [unbounded_token.format(), space(), direction.format()])
    }
}
