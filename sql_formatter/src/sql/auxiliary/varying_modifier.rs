use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlVaryingModifier;
use sql_syntax::SqlVaryingModifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlVaryingModifier;
impl FormatNodeRule<SqlVaryingModifier> for FormatSqlVaryingModifier {
    fn fmt_fields(&self, node: &SqlVaryingModifier, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlVaryingModifierFields { varying_token } = node.as_fields();

        write!(f, [varying_token.format()])
    }
}
