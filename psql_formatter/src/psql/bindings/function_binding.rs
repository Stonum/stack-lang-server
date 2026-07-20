use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use psql_syntax::PsqlFunctionBinding;
use psql_syntax::PsqlFunctionBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlFunctionBinding;
impl FormatNodeRule<PsqlFunctionBinding> for FormatPsqlFunctionBinding {
    fn fmt_fields(&self, node: &PsqlFunctionBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlFunctionBindingFields {
            schema,
            name,
            l_paren_token,
            arguments,
            r_paren_token,
            alias,
        } = node.as_fields();

        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])?;
        write_bracketed_fill_list(l_paren_token, &arguments, r_paren_token, f)?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
