use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlSecurityOption;
use sql_syntax::PsqlSecurityOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSecurityOption;
impl FormatNodeRule<PsqlSecurityOption> for FormatPsqlSecurityOption {
    fn fmt_fields(&self, node: &PsqlSecurityOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlSecurityOptionFields {
            security_token,
            value,
        } = node.as_fields();

        write!(f, [security_token.format(), space(), value.format()])
    }
}
