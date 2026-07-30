use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlParameterDefault;
use psql_syntax::PsqlParameterDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlParameterDefault;
impl FormatNodeRule<PsqlParameterDefault> for FormatPsqlParameterDefault {
    fn fmt_fields(&self, node: &PsqlParameterDefault, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlParameterDefaultFields {
            default_token,
            value,
        } = node.as_fields();

        write!(f, [default_token.format(), space(), value.format()])
    }
}
