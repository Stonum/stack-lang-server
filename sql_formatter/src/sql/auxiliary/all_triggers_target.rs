use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAllTriggersTarget;
use sql_syntax::SqlAllTriggersTargetFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAllTriggersTarget;
impl FormatNodeRule<SqlAllTriggersTarget> for FormatSqlAllTriggersTarget {
    fn fmt_fields(&self, node: &SqlAllTriggersTarget, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAllTriggersTargetFields { scope } = node.as_fields();

        write!(f, [scope.format()])
    }
}
