use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlConstraintName;
use sql_syntax::SqlConstraintNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlConstraintName;
impl FormatNodeRule<SqlConstraintName> for FormatSqlConstraintName {
    fn fmt_fields(&self, node: &SqlConstraintName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlConstraintNameFields {
            constraint_token,
            name,
        } = node.as_fields();

        write!(f, [constraint_token.format(), space(), name.format()])
    }
}
