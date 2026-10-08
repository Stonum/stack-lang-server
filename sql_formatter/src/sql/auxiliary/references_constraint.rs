use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlReferencesConstraint;
use sql_syntax::SqlReferencesConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlReferencesConstraint;
impl FormatNodeRule<SqlReferencesConstraint> for FormatSqlReferencesConstraint {
    fn fmt_fields(&self, node: &SqlReferencesConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlReferencesConstraintFields {
            references_token,
            table,
            columns,
        } = node.as_fields();

        write!(f, [references_token.format(), space(), table.format()])?;
        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }
        Ok(())
    }
}
