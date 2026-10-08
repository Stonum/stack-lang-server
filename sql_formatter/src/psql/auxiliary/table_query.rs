use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTableQuery;
use sql_syntax::PsqlTableQueryFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableQuery;
impl FormatNodeRule<PsqlTableQuery> for FormatPsqlTableQuery {
    fn fmt_fields(&self, node: &PsqlTableQuery, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlTableQueryFields { table_token, table } = node.as_fields();

        write!(f, [table_token.format(), space(), table.format()])
    }
}
