use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlFunctionParameter;
use psql_syntax::PsqlFunctionParameterFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionParameter;
impl FormatNodeRule<PsqlFunctionParameter> for FormatPsqlFunctionParameter {
    fn fmt_fields(&self, node: &PsqlFunctionParameter, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFunctionParameterFields {
            mode,
            name,
            ty,
            default,
        } = node.as_fields();

        if let Some(mode) = mode {
            write!(f, [mode.format(), space()])?;
        }

        write!(f, [name.format(), space(), ty.format()])?;

        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }
        Ok(())
    }
}
