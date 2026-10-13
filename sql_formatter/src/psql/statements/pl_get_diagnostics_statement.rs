use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::PsqlPlDiagnosticsItem;
use sql_syntax::PsqlPlGetDiagnosticsStatement;
use sql_syntax::PsqlPlGetDiagnosticsStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlGetDiagnosticsStatement;
impl FormatNodeRule<PsqlPlGetDiagnosticsStatement> for FormatPsqlPlGetDiagnosticsStatement {
    fn fmt_fields(
        &self,
        node: &PsqlPlGetDiagnosticsStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPlGetDiagnosticsStatementFields {
            get_token,
            scope,
            diagnostics_token,
            items,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [get_token.format(), space()])?;
            if let Some(scope) = &scope {
                write!(f, [scope.format(), space()])?;
            }
            write!(f, [diagnostics_token.format()])
        });

        write!(
            f,
            [group(&format_once(|f| {
                write_wrapping_fill_clause(keyword, &items, |_: &PsqlPlDiagnosticsItem| false, f)
            }))]
        )?;
        write!(f, [semicolon_token.format()])
    }
}
