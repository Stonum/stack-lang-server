use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlSubqueryBinding;
use sql_syntax::SqlSubqueryBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSubqueryBinding;
impl FormatNodeRule<SqlSubqueryBinding> for FormatSqlSubqueryBinding {
    fn fmt_fields(&self, node: &SqlSubqueryBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSubqueryBindingFields {
            lateral_token,
            l_paren_token,
            query,
            r_paren_token,
            alias,
        } = node.as_fields();

        if let Some(lateral_token) = lateral_token {
            write!(f, [lateral_token.format(), space()])?;
        }
        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&query.format())),
                r_paren_token.format(),
            ]
        )?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
