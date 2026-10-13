use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::PsqlPlForStatement;
use sql_syntax::PsqlPlForStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForStatement;
impl FormatNodeRule<PsqlPlForStatement> for FormatPsqlPlForStatement {
    fn fmt_fields(&self, node: &PsqlPlForStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlForStatementFields {
            label,
            for_token,
            targets,
            in_token,
            source,
            body,
        } = node.as_fields();

        if let Some(label) = label {
            write!(f, [label.format(), hard_line_break()])?;
        }
        // A source that does not fit moves under `for ... in`, with `loop`
        // on its own line.
        write!(
            f,
            [
                group(&format_args![
                    for_token.format(),
                    space(),
                    targets.format(),
                    space(),
                    in_token.format(),
                    indent(&format_args![soft_line_break_or_space(), source.format()]),
                    soft_line_break_or_space()
                ]),
                body.format()
            ]
        )
    }
}
