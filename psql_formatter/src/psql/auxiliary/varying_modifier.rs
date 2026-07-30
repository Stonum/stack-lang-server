use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlVaryingModifier;
use psql_syntax::PsqlVaryingModifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlVaryingModifier;
impl FormatNodeRule<PsqlVaryingModifier> for FormatPsqlVaryingModifier {
    fn fmt_fields(&self, node: &PsqlVaryingModifier, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlVaryingModifierFields { varying_token } = node.as_fields();

        write!(f, [varying_token.format()])
    }
}
