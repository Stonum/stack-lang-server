use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlFetchDirection;
use sql_syntax::PsqlPlFetchDirectionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlFetchDirection;
impl FormatNodeRule<PsqlPlFetchDirection> for FormatPsqlPlFetchDirection {
    fn fmt_fields(&self, node: &PsqlPlFetchDirection, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlFetchDirectionFields {
            kind,
            all_token,
            count,
        } = node.as_fields();

        let mut needs_space = false;
        if let Some(kind) = kind {
            write!(f, [kind.format()])?;
            needs_space = true;
        }
        if let Some(all_token) = all_token {
            if needs_space {
                write!(f, [space()])?;
            }
            write!(f, [all_token.format()])?;
            needs_space = true;
        }
        if let Some(count) = count {
            if needs_space {
                write!(f, [space()])?;
            }
            write!(f, [count.format()])?;
        }
        Ok(())
    }
}
