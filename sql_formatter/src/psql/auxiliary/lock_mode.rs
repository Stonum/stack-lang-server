use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlLockMode;
use sql_syntax::PsqlLockModeFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLockMode;
impl FormatNodeRule<PsqlLockMode> for FormatPsqlLockMode {
    fn fmt_fields(&self, node: &PsqlLockMode, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlLockModeFields {
            in_token,
            lead,
            middle,
            tail,
            mode_token,
        } = node.as_fields();

        write!(f, [in_token.format(), space(), lead.format()])?;
        for token in [&middle, &tail].into_iter().flatten() {
            write!(f, [space(), token.format()])?;
        }
        write!(f, [space(), mode_token.format()])
    }
}
