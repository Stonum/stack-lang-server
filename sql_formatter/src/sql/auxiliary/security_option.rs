use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSecurityOption;
use sql_syntax::SqlSecurityOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSecurityOption;
impl FormatNodeRule<SqlSecurityOption> for FormatSqlSecurityOption {
    fn fmt_fields(&self, node: &SqlSecurityOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSecurityOptionFields {
            security_token,
            value,
        } = node.as_fields();

        write!(f, [security_token.format(), space(), value.format()])
    }
}
