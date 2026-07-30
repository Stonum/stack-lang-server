use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlVolatilityOption;
use psql_syntax::PsqlVolatilityOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlVolatilityOption;
impl FormatNodeRule<PsqlVolatilityOption> for FormatPsqlVolatilityOption {
    fn fmt_fields(&self, node: &PsqlVolatilityOption, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlVolatilityOptionFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
