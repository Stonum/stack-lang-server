use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlName;
use psql_syntax::PsqlNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlName;
impl FormatNodeRule<PsqlName> for FormatPsqlName {
    fn fmt_fields(&self, node: &PsqlName, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlNameFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
