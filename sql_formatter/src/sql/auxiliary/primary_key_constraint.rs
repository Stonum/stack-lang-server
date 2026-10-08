use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPrimaryKeyConstraint;
use sql_syntax::SqlPrimaryKeyConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrimaryKeyConstraint;
impl FormatNodeRule<SqlPrimaryKeyConstraint> for FormatSqlPrimaryKeyConstraint {
    fn fmt_fields(&self, node: &SqlPrimaryKeyConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPrimaryKeyConstraintFields {
            primary_token,
            key_token,
        } = node.as_fields();

        write!(f, [primary_token.format(), space(), key_token.format()])
    }
}
