use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlFunctionParameter;
use psql_syntax::PsqlFunctionParameterFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionParameter;
impl FormatNodeRule<PsqlFunctionParameter> for FormatPsqlFunctionParameter {
    fn fmt_fields(&self, node: &PsqlFunctionParameter, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFunctionParameterFields { name, ty } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])
    }
}
