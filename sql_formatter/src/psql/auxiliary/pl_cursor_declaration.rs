use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlCursorDeclaration;
use sql_syntax::PsqlPlCursorDeclarationFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCursorDeclaration;
impl FormatNodeRule<PsqlPlCursorDeclaration> for FormatPsqlPlCursorDeclaration {
    fn fmt_fields(&self, node: &PsqlPlCursorDeclaration, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlCursorDeclarationFields {
            name,
            scroll,
            cursor_token,
            parameters,
            keyword,
            query,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(scroll) = scroll {
            write!(f, [space(), scroll.format()])?;
        }
        write!(f, [space(), cursor_token.format()])?;
        if let Some(parameters) = parameters {
            write!(f, [space(), parameters.format()])?;
        }
        // A query that does not fit moves under the header, indented.
        write!(
            f,
            [
                space(),
                keyword.format(),
                group(&soft_line_indent_or_space(&query.format()))
            ]
        )
    }
}
