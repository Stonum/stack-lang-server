use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlStrictOption;
use psql_syntax::PsqlStrictOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStrictOption;
impl FormatNodeRule<PsqlStrictOption> for FormatPsqlStrictOption {
    fn fmt_fields(&self, node: &PsqlStrictOption, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlStrictOptionFields { strict_token } = node.as_fields();

        write!(f, [strict_token.format()])
    }
}
