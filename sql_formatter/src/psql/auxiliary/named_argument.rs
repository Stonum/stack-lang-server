use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlNamedArgument;
use sql_syntax::PsqlNamedArgumentFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlNamedArgument;
impl FormatNodeRule<PsqlNamedArgument> for FormatPsqlNamedArgument {
    fn fmt_fields(&self, node: &PsqlNamedArgument, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlNamedArgumentFields {
            name,
            operator,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                space(),
                operator.format(),
                space(),
                value.format()
            ]
        )
    }
}
