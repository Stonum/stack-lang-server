use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTableName;
use psql_syntax::PsqlTableNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableName;
impl FormatNodeRule<PsqlTableName> for FormatPsqlTableName {
    fn fmt_fields(&self, node: &PsqlTableName, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTableNameFields { schema, name } = node.as_fields();

        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])
    }
}
