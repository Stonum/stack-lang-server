use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::SqlFunctionBinding;
use sql_syntax::SqlFunctionBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlFunctionBinding;
impl FormatNodeRule<SqlFunctionBinding> for FormatSqlFunctionBinding {
    fn fmt_fields(&self, node: &SqlFunctionBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlFunctionBindingFields {
            lateral_token,
            schema,
            name,
            l_paren_token,
            arguments,
            r_paren_token,
            alias,
        } = node.as_fields();

        if let Some(lateral_token) = lateral_token {
            write!(f, [lateral_token.format(), space()])?;
        }
        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])?;
        write_bracketed_fill_list(
            l_paren_token,
            &arguments,
            r_paren_token,
            |expr: &AnySqlExpression| !is_simple_expression(expr, 0),
            f,
        )?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
