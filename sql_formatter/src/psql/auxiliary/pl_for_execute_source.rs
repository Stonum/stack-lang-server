use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlForExecuteSource;
use sql_syntax::PsqlPlForExecuteSourceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlForExecuteSource;
impl FormatNodeRule<PsqlPlForExecuteSource> for FormatPsqlPlForExecuteSource {
    fn fmt_fields(&self, node: &PsqlPlForExecuteSource, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlForExecuteSourceFields {
            execute_token,
            command,
            using_clause,
        } = node.as_fields();

        write!(f, [execute_token.format(), space(), command.format()])?;
        if let Some(using_clause) = using_clause {
            write!(f, [soft_line_break_or_space(), using_clause.format()])?;
        }
        Ok(())
    }
}
