use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::{format_args, write};
use sql_syntax::PsqlDropMaterializedViewStatement;
use sql_syntax::PsqlDropMaterializedViewStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropMaterializedViewStatement;
impl FormatNodeRule<PsqlDropMaterializedViewStatement> for FormatPsqlDropMaterializedViewStatement {
    fn fmt_fields(
        &self,
        node: &PsqlDropMaterializedViewStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDropMaterializedViewStatementFields {
            drop_token,
            materialized_token,
            view_token,
            if_token,
            exists_token,
            views,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(
                f,
                [
                    drop_token.format(),
                    space(),
                    materialized_token.format(),
                    space(),
                    view_token.format()
                ]
            )?;
            for token in [&if_token, &exists_token].into_iter().flatten() {
                write!(f, [space(), token.format()])?;
            }
            Ok(())
        });
        let list = format_with(|f| write_wrapping_fill_clause(&keyword, &views, |_| false, f));

        match drop_behavior {
            // Once the view list wraps, `CASCADE`/`RESTRICT` moves to its own line.
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
