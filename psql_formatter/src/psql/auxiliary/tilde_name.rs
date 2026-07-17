use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTildeName;
use psql_syntax::PsqlTildeNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTildeName;
impl FormatNodeRule<PsqlTildeName> for FormatPsqlTildeName {
    fn fmt_fields(&self, node: &PsqlTildeName, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTildeNameFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
