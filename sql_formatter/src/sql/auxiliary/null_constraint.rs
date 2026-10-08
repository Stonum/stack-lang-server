use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlNullConstraint;
use sql_syntax::SqlNullConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlNullConstraint;
impl FormatNodeRule<SqlNullConstraint> for FormatSqlNullConstraint {
    fn fmt_fields(&self, node: &SqlNullConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlNullConstraintFields { null_token } = node.as_fields();

        write!(f, [null_token.format()])
    }
}
