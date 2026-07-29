use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlColumnDefinition;
use psql_syntax::PsqlColumnDefinitionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlColumnDefinition;
impl FormatNodeRule<PsqlColumnDefinition> for FormatPsqlColumnDefinition {
    fn fmt_fields(&self, node: &PsqlColumnDefinition, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlColumnDefinitionFields { name, ty } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])
    }
}
