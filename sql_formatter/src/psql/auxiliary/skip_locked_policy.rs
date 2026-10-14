use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlSkipLockedPolicy;
use sql_syntax::PsqlSkipLockedPolicyFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSkipLockedPolicy;
impl FormatNodeRule<PsqlSkipLockedPolicy> for FormatPsqlSkipLockedPolicy {
    fn fmt_fields(&self, node: &PsqlSkipLockedPolicy, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlSkipLockedPolicyFields {
            skip_token,
            locked_token,
        } = node.as_fields();

        write!(f, [skip_token.format(), space(), locked_token.format()])
    }
}
