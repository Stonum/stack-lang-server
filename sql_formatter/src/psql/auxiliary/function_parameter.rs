use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlFunctionParameter;
use sql_syntax::PsqlFunctionParameterFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionParameter;
impl FormatNodeRule<PsqlFunctionParameter> for FormatPsqlFunctionParameter {
    fn fmt_fields(&self, node: &PsqlFunctionParameter, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlFunctionParameterFields {
            mode,
            name,
            ty,
            default,
        } = node.as_fields();

        if let Some(mode) = mode {
            write!(f, [mode.format(), space()])?;
        }

        if let Some(name) = name {
            write!(f, [name.format(), space()])?;
        }

        write!(f, [ty.format()])?;

        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }
        Ok(())
    }
}
