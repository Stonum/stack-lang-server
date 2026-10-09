use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCopyTableSource;
use sql_syntax::PsqlCopyTableSourceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyTableSource;
impl FormatNodeRule<PsqlCopyTableSource> for FormatPsqlCopyTableSource {
    fn fmt_fields(&self, node: &PsqlCopyTableSource, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCopyTableSourceFields { table, columns } = node.as_fields();

        write!(f, [table.format()])?;
        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }
        Ok(())
    }
}
