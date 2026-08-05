use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPrecisionModifier;
use sql_syntax::SqlPrecisionModifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrecisionModifier;
impl FormatNodeRule<SqlPrecisionModifier> for FormatSqlPrecisionModifier {
    fn fmt_fields(&self, node: &SqlPrecisionModifier, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPrecisionModifierFields { precision_token } = node.as_fields();

        write!(f, [precision_token.format()])
    }
}
