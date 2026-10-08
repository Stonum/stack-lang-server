use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlNotNullConstraint;
use sql_syntax::SqlNotNullConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlNotNullConstraint;
impl FormatNodeRule<SqlNotNullConstraint> for FormatSqlNotNullConstraint {
    fn fmt_fields(&self, node: &SqlNotNullConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlNotNullConstraintFields {
            not_token,
            null_token,
        } = node.as_fields();

        write!(f, [not_token.format(), space(), null_token.format()])
    }
}
