use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCheckConstraint;
use sql_syntax::SqlCheckConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCheckConstraint;
impl FormatNodeRule<SqlCheckConstraint> for FormatSqlCheckConstraint {
    fn fmt_fields(&self, node: &SqlCheckConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCheckConstraintFields {
            check_token,
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                check_token.format(),
                space(),
                l_paren_token.format(),
                condition.format(),
                r_paren_token.format()
            ]
        )
    }
}
