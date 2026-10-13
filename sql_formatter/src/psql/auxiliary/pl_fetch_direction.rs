use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlFetchDirection;
use sql_syntax::PsqlPlFetchDirectionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlFetchDirection;
impl FormatNodeRule<PsqlPlFetchDirection> for FormatPsqlPlFetchDirection {
    fn fmt_fields(&self, node: &PsqlPlFetchDirection, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlFetchDirectionFields { kind, count } = node.as_fields();

        if let Some(kind) = &kind {
            write!(f, [kind.format()])?;
        }
        if let Some(count) = count {
            if kind.is_some() {
                write!(f, [space()])?;
            }
            write!(f, [count.format()])?;
        }
        Ok(())
    }
}
