use crate::prelude::*;
use sql_syntax::SqlColumnConstraintList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlColumnConstraintList;
impl FormatRule<SqlColumnConstraintList> for FormatSqlColumnConstraintList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &SqlColumnConstraintList, f: &mut SqlFormatter) -> FormatResult<()> {
        // Constraints that don't fit continue on indented lines; only the
        // continuation lines are indented, so a constraint that breaks on the
        // column's own line (e.g. a long `CHECK (...)`) stays aligned with it.
        let mut filler = f.fill();
        for (index, constraint) in node.iter().enumerate() {
            if index == 0 {
                filler.entry(&soft_line_break_or_space(), &constraint.format());
            } else {
                filler.entry(
                    &indent(&soft_line_break_or_space()),
                    &indent(&constraint.format()),
                );
            }
        }
        filler.finish()
    }
}
