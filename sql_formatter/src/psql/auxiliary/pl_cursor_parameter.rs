use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlCursorParameter;
use sql_syntax::PsqlPlCursorParameterFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlCursorParameter;
impl FormatNodeRule<PsqlPlCursorParameter> for FormatPsqlPlCursorParameter {
    fn fmt_fields(&self, node: &PsqlPlCursorParameter, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlCursorParameterFields { name, ty } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])
    }
}
