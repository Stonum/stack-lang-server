use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSubqueryBinding;
use psql_syntax::PsqlSubqueryBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubqueryBinding;
impl FormatNodeRule<PsqlSubqueryBinding> for FormatPsqlSubqueryBinding {
    fn fmt_fields(&self, node: &PsqlSubqueryBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSubqueryBindingFields {
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
