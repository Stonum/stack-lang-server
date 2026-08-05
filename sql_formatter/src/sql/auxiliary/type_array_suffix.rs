use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTypeArraySuffix;
use sql_syntax::SqlTypeArraySuffixFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTypeArraySuffix;
impl FormatNodeRule<SqlTypeArraySuffix> for FormatSqlTypeArraySuffix {
    fn fmt_fields(&self, node: &SqlTypeArraySuffix, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTypeArraySuffixFields {
            l_brack_token,
            r_brack_token,
        } = node.as_fields();

        write!(f, [l_brack_token.format(), r_brack_token.format()])
    }
}
