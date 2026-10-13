use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlWindowFrameCurrentRowBound;
use sql_syntax::SqlWindowFrameCurrentRowBoundFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowFrameCurrentRowBound;
impl FormatNodeRule<SqlWindowFrameCurrentRowBound> for FormatSqlWindowFrameCurrentRowBound {
    fn fmt_fields(
        &self,
        node: &SqlWindowFrameCurrentRowBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlWindowFrameCurrentRowBoundFields {
            current_token,
            row_token,
        } = node.as_fields();

        write!(f, [current_token.format(), space(), row_token.format()])
    }
}
