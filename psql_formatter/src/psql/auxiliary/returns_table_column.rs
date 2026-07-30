use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlReturnsTableColumn;
use psql_syntax::PsqlReturnsTableColumnFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlReturnsTableColumn;
impl FormatNodeRule<PsqlReturnsTableColumn> for FormatPsqlReturnsTableColumn {
    fn fmt_fields(&self, node: &PsqlReturnsTableColumn, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlReturnsTableColumnFields { name, ty } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])
    }
}
