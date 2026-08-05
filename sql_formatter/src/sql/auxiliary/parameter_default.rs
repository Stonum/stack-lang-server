use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlParameterDefault;
use sql_syntax::SqlParameterDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlParameterDefault;
impl FormatNodeRule<SqlParameterDefault> for FormatSqlParameterDefault {
    fn fmt_fields(&self, node: &SqlParameterDefault, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlParameterDefaultFields { marker, value } = node.as_fields();

        write!(f, [marker.format(), space(), value.format()])
    }
}
