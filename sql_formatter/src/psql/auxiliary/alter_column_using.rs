use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlAlterColumnUsing;
use sql_syntax::PsqlAlterColumnUsingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAlterColumnUsing;
impl FormatNodeRule<PsqlAlterColumnUsing> for FormatPsqlAlterColumnUsing {
    fn fmt_fields(&self, node: &PsqlAlterColumnUsing, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlAlterColumnUsingFields {
            using_token,
            expression,
        } = node.as_fields();

        write!(f, [using_token.format(), space(), expression.format()])
    }
}
