use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTableBinding;
use sql_syntax::SqlTableBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableBinding;
impl FormatNodeRule<SqlTableBinding> for FormatSqlTableBinding {
    fn fmt_fields(&self, node: &SqlTableBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTableBindingFields { table, alias } = node.as_fields();

        write!(f, [table.format()])?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
