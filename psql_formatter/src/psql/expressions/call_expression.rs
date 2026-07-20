use crate::prelude::*;
use crate::utils::write_bracketed_fill_list;
use biome_formatter::write;
use psql_syntax::PsqlCallExpression;
use psql_syntax::PsqlCallExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCallExpression;
impl FormatNodeRule<PsqlCallExpression> for FormatPsqlCallExpression {
    fn fmt_fields(&self, node: &PsqlCallExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlCallExpressionFields {
            schema,
            name,
            l_paren_token,
            arguments,
            r_paren_token,
        } = node.as_fields();

        if let Some(schema) = schema {
            write!(f, [schema.format()])?;
        }
        write!(f, [name.format()])?;
        write_bracketed_fill_list(l_paren_token, &arguments, r_paren_token, f)
    }
}
