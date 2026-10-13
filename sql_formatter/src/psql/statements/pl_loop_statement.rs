use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlLoopStatement;
use sql_syntax::PsqlPlLoopStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlLoopStatement;
impl FormatNodeRule<PsqlPlLoopStatement> for FormatPsqlPlLoopStatement {
    fn fmt_fields(&self, node: &PsqlPlLoopStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlLoopStatementFields { label, body } = node.as_fields();

        if let Some(label) = label {
            write!(f, [label.format(), hard_line_break()])?;
        }
        write!(f, [body.format()])
    }
}
