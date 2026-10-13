use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlWhileStatement;
use sql_syntax::PsqlPlWhileStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlWhileStatement;
impl FormatNodeRule<PsqlPlWhileStatement> for FormatPsqlPlWhileStatement {
    fn fmt_fields(&self, node: &PsqlPlWhileStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlWhileStatementFields {
            label,
            while_token,
            condition,
            body,
        } = node.as_fields();

        if let Some(label) = label {
            write!(f, [label.format(), hard_line_break()])?;
        }
        write!(
            f,
            [
                while_token.format(),
                space(),
                condition.format(),
                space(),
                body.format()
            ]
        )
    }
}
