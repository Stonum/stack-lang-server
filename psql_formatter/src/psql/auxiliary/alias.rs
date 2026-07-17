use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlAlias;
use psql_syntax::PsqlAliasFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAlias;
impl FormatNodeRule<PsqlAlias> for FormatPsqlAlias {
    fn fmt_fields(&self, node: &PsqlAlias, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlAliasFields { as_token, value } = node.as_fields();

        if let Some(as_token) = as_token {
            write!(f, [as_token.format(), space()])?;
        }
        write!(f, [value.format()])
    }
}
