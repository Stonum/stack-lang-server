use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlUniqueTableConstraint;
use sql_syntax::SqlUniqueTableConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlUniqueTableConstraint;
impl FormatNodeRule<SqlUniqueTableConstraint> for FormatSqlUniqueTableConstraint {
    fn fmt_fields(
        &self,
        node: &SqlUniqueTableConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlUniqueTableConstraintFields {
            unique_token,
            columns,
        } = node.as_fields();

        write!(f, [unique_token.format(), space(), columns.format()])
    }
}
