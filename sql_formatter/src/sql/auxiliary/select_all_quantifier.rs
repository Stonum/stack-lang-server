use crate::prelude::*;
use sql_syntax::SqlSelectAllQuantifier;
use sql_syntax::SqlSelectAllQuantifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSelectAllQuantifier;
impl FormatNodeRule<SqlSelectAllQuantifier> for FormatSqlSelectAllQuantifier {
    fn fmt_fields(&self, node: &SqlSelectAllQuantifier, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSelectAllQuantifierFields { all_token } = node.as_fields();

        all_token.format().fmt(f)
    }
}
