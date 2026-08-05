use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlFunctionParameter;
use sql_syntax::SqlFunctionParameterFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFunctionParameter;
impl FormatNodeRule<SqlFunctionParameter> for FormatSqlFunctionParameter {
    fn fmt_fields(&self, node: &SqlFunctionParameter, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFunctionParameterFields {
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
