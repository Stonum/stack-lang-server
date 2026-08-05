use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAliasColumnDefinition;
use sql_syntax::SqlAliasColumnDefinitionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAliasColumnDefinition;
impl FormatNodeRule<SqlAliasColumnDefinition> for FormatSqlAliasColumnDefinition {
    fn fmt_fields(
        &self,
        node: &SqlAliasColumnDefinition,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlAliasColumnDefinitionFields { name, ty } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(ty) = ty {
            write!(f, [space(), ty.format()])?;
        }
        Ok(())
    }
}
