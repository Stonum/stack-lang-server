use crate::prelude::*;
use biome_formatter::{format_args, write};
use psql_syntax::PsqlParenthesizedJoinBinding;
use psql_syntax::PsqlParenthesizedJoinBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlParenthesizedJoinBinding;
impl FormatNodeRule<PsqlParenthesizedJoinBinding> for FormatPsqlParenthesizedJoinBinding {
    fn fmt_fields(
        &self,
        node: &PsqlParenthesizedJoinBinding,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlParenthesizedJoinBindingFields {
            l_paren_token,
            source,
            joins,
            r_paren_token,
            alias,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&format_args![
                    source.format(),
                    joins.format()
                ])),
                r_paren_token.format(),
            ]
        )?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
