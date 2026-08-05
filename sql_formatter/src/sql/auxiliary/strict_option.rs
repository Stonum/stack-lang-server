use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlStrictOption;
use sql_syntax::SqlStrictOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlStrictOption;
impl FormatNodeRule<SqlStrictOption> for FormatSqlStrictOption {
    fn fmt_fields(&self, node: &SqlStrictOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlStrictOptionFields { strict_token } = node.as_fields();

        write!(f, [strict_token.format()])
    }
}
