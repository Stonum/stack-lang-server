use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTypeArraySuffix;
use psql_syntax::PsqlTypeArraySuffixFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeArraySuffix;
impl FormatNodeRule<PsqlTypeArraySuffix> for FormatPsqlTypeArraySuffix {
    fn fmt_fields(&self, node: &PsqlTypeArraySuffix, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTypeArraySuffixFields {
            l_brack_token,
            r_brack_token,
        } = node.as_fields();

        write!(f, [l_brack_token.format(), r_brack_token.format()])
    }
}
