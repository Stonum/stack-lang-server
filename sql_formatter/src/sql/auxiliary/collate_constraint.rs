use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCollateConstraint;
use sql_syntax::SqlCollateConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCollateConstraint;
impl FormatNodeRule<SqlCollateConstraint> for FormatSqlCollateConstraint {
    fn fmt_fields(&self, node: &SqlCollateConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCollateConstraintFields {
            collate_token,
            collation,
        } = node.as_fields();

        write!(f, [collate_token.format(), space(), collation.format()])
    }
}
