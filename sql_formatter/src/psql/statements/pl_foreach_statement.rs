use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::PsqlPlForeachStatement;
use sql_syntax::PsqlPlForeachStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForeachStatement;
impl FormatNodeRule<PsqlPlForeachStatement> for FormatPsqlPlForeachStatement {
    fn fmt_fields(&self, node: &PsqlPlForeachStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlForeachStatementFields {
            label,
            foreach_token,
            targets,
            slice,
            in_token,
            array_token,
            array,
            body,
        } = node.as_fields();

        if let Some(label) = label {
            write!(f, [label.format(), hard_line_break()])?;
        }
        // Same layout as `FOR` (`pl_for_statement.rs`).
        write!(
            f,
            [
                group(&format_args![
                    foreach_token.format(),
                    space(),
                    targets.format(),
                    format_with(|f| match &slice {
                        Some(slice) => write!(f, [space(), slice.format()]),
                        None => Ok(()),
                    }),
                    space(),
                    in_token.format(),
                    space(),
                    array_token.format(),
                    indent(&format_args![soft_line_break_or_space(), array.format()]),
                    soft_line_break_or_space()
                ]),
                body.format()
            ]
        )
    }
}
