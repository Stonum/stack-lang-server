use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlNowaitPolicy;
use sql_syntax::PsqlNowaitPolicyFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlNowaitPolicy;
impl FormatNodeRule<PsqlNowaitPolicy> for FormatPsqlNowaitPolicy {
    fn fmt_fields(&self, node: &PsqlNowaitPolicy, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlNowaitPolicyFields { nowait_token } = node.as_fields();

        write!(f, [nowait_token.format()])
    }
}
