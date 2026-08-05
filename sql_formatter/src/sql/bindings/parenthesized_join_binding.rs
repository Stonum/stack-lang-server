use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::SqlParenthesizedJoinBinding;
use sql_syntax::SqlParenthesizedJoinBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlParenthesizedJoinBinding;
impl FormatNodeRule<SqlParenthesizedJoinBinding> for FormatSqlParenthesizedJoinBinding {
    fn fmt_fields(
        &self,
        node: &SqlParenthesizedJoinBinding,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let SqlParenthesizedJoinBindingFields {
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
