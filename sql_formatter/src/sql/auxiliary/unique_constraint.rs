use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlUniqueConstraint;
use sql_syntax::SqlUniqueConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlUniqueConstraint;
impl FormatNodeRule<SqlUniqueConstraint> for FormatSqlUniqueConstraint {
    fn fmt_fields(&self, node: &SqlUniqueConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlUniqueConstraintFields { unique_token } = node.as_fields();

        write!(f, [unique_token.format()])
    }
}
