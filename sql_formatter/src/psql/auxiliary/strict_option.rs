use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlStrictOption;
use sql_syntax::PsqlStrictOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStrictOption;
impl FormatNodeRule<PsqlStrictOption> for FormatPsqlStrictOption {
    fn fmt_fields(&self, node: &PsqlStrictOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlStrictOptionFields { strict_token } = node.as_fields();

        write!(f, [strict_token.format()])
    }
}
