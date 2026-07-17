use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDataBaseName;
use psql_syntax::PsqlDataBaseNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDataBaseName;
impl FormatNodeRule<PsqlDataBaseName> for FormatPsqlDataBaseName {
    fn fmt_fields(&self, node: &PsqlDataBaseName, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlDataBaseNameFields { name, dot_token } = node.as_fields();

        write!(f, [name.format(), dot_token.format()])
    }
}
