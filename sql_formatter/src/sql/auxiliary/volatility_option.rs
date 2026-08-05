use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlVolatilityOption;
use sql_syntax::SqlVolatilityOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlVolatilityOption;
impl FormatNodeRule<SqlVolatilityOption> for FormatSqlVolatilityOption {
    fn fmt_fields(&self, node: &SqlVolatilityOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlVolatilityOptionFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
