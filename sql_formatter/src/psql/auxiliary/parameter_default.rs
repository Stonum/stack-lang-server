use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlParameterDefault;
use sql_syntax::PsqlParameterDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlParameterDefault;
impl FormatNodeRule<PsqlParameterDefault> for FormatPsqlParameterDefault {
    fn fmt_fields(&self, node: &PsqlParameterDefault, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlParameterDefaultFields { marker, value } = node.as_fields();

        write!(f, [marker.format(), space(), value.format()])
    }
}
