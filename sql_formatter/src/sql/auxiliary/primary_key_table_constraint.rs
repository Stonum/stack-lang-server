use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPrimaryKeyTableConstraint;
use sql_syntax::SqlPrimaryKeyTableConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrimaryKeyTableConstraint;
impl FormatNodeRule<SqlPrimaryKeyTableConstraint> for FormatSqlPrimaryKeyTableConstraint {
    fn fmt_fields(
        &self,
        node: &SqlPrimaryKeyTableConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlPrimaryKeyTableConstraintFields {
            primary_token,
            key_token,
            columns,
        } = node.as_fields();

        write!(
            f,
            [
                primary_token.format(),
                space(),
                key_token.format(),
                space(),
                columns.format()
            ]
        )
    }
}
