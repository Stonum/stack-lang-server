use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTableBinding;
use psql_syntax::PsqlTableBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableBinding;
impl FormatNodeRule<PsqlTableBinding> for FormatPsqlTableBinding {
    fn fmt_fields(&self, node: &PsqlTableBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTableBindingFields { table, alias } = node.as_fields();

        write!(f, [table.format()])?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
