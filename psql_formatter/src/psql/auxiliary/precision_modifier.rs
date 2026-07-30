use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlPrecisionModifier;
use psql_syntax::PsqlPrecisionModifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPrecisionModifier;
impl FormatNodeRule<PsqlPrecisionModifier> for FormatPsqlPrecisionModifier {
    fn fmt_fields(&self, node: &PsqlPrecisionModifier, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlPrecisionModifierFields { precision_token } = node.as_fields();

        write!(f, [precision_token.format()])
    }
}
