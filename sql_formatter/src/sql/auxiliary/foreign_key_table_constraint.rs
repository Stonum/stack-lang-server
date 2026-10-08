use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlForeignKeyTableConstraint;
use sql_syntax::SqlForeignKeyTableConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlForeignKeyTableConstraint;
impl FormatNodeRule<SqlForeignKeyTableConstraint> for FormatSqlForeignKeyTableConstraint {
    fn fmt_fields(
        &self,
        node: &SqlForeignKeyTableConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlForeignKeyTableConstraintFields {
            foreign_token,
            key_token,
            columns,
            references,
        } = node.as_fields();

        write!(
            f,
            [
                foreign_token.format(),
                space(),
                key_token.format(),
                space(),
                columns.format(),
                space(),
                references.format()
            ]
        )
    }
}
