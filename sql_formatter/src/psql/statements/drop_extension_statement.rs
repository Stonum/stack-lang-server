use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::{format_args, write};
use sql_syntax::PsqlDropExtensionStatement;
use sql_syntax::PsqlDropExtensionStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropExtensionStatement;
impl FormatNodeRule<PsqlDropExtensionStatement> for FormatPsqlDropExtensionStatement {
    fn fmt_fields(
        &self,
        node: &PsqlDropExtensionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDropExtensionStatementFields {
            drop_token,
            extension_token,
            if_token,
            exists_token,
            extensions,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(f, [drop_token.format(), space(), extension_token.format()])?;
            for token in [&if_token, &exists_token].into_iter().flatten() {
                write!(f, [space(), token.format()])?;
            }
            Ok(())
        });
        let list = format_with(|f| write_wrapping_fill_clause(&keyword, &extensions, |_| false, f));

        match drop_behavior {
            // Once the extension list wraps, `CASCADE`/`RESTRICT` moves to its own line.
            Some(drop_behavior) => write!(
                f,
                [group(&format_args![
                    list,
                    soft_line_break_or_space(),
                    drop_behavior.format()
                ])]
            )?,
            None => write!(f, [list])?,
        }

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
