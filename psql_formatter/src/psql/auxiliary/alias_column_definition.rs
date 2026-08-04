use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlAliasColumnDefinition;
use psql_syntax::PsqlAliasColumnDefinitionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAliasColumnDefinition;
impl FormatNodeRule<PsqlAliasColumnDefinition> for FormatPsqlAliasColumnDefinition {
    fn fmt_fields(
        &self,
        node: &PsqlAliasColumnDefinition,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlAliasColumnDefinitionFields { name, ty } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(ty) = ty {
            write!(f, [space(), ty.format()])?;
        }
        Ok(())
    }
}
