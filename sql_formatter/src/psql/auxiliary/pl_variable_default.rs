use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlVariableDefault;
use sql_syntax::PsqlPlVariableDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlVariableDefault;
impl FormatNodeRule<PsqlPlVariableDefault> for FormatPsqlPlVariableDefault {
    fn fmt_fields(&self, node: &PsqlPlVariableDefault, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlVariableDefaultFields { operator, value } = node.as_fields();

        write!(f, [operator.format(), space(), value.format()])
    }
}
