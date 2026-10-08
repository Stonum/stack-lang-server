use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTableAndColumns;
use sql_syntax::PsqlTableAndColumnsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableAndColumns;
impl FormatNodeRule<PsqlTableAndColumns> for FormatPsqlTableAndColumns {
    fn fmt_fields(&self, node: &PsqlTableAndColumns, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlTableAndColumnsFields { table, columns } = node.as_fields();

        write!(f, [table.format()])?;
        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }
        Ok(())
    }
}
