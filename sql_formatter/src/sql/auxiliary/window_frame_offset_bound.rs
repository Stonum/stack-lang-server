use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWindowFrameOffsetBound;
use sql_syntax::SqlWindowFrameOffsetBoundFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowFrameOffsetBound;
impl FormatNodeRule<SqlWindowFrameOffsetBound> for FormatSqlWindowFrameOffsetBound {
    fn fmt_fields(
        &self,
        node: &SqlWindowFrameOffsetBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlWindowFrameOffsetBoundFields { offset, direction } = node.as_fields();

        write!(f, [offset.format(), space(), direction.format()])
    }
}
