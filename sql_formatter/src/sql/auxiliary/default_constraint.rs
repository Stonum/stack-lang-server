use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDefaultConstraint;
use sql_syntax::SqlDefaultConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDefaultConstraint;
impl FormatNodeRule<SqlDefaultConstraint> for FormatSqlDefaultConstraint {
    fn fmt_fields(&self, node: &SqlDefaultConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDefaultConstraintFields {
            default_token,
            value,
        } = node.as_fields();

        write!(f, [default_token.format(), space(), value.format()])
    }
}
