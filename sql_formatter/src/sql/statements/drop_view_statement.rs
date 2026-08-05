use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropViewStatement;
use sql_syntax::SqlDropViewStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropViewStatement;
impl FormatNodeRule<SqlDropViewStatement> for FormatSqlDropViewStatement {
    fn fmt_fields(&self, node: &SqlDropViewStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropViewStatementFields {
            drop_token,
            view_token,
            if_token,
            exists_token,
            views,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), view_token.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(exists_token) = exists_token {
            write!(f, [space(), exists_token.format()])?;
        }

        if views.len() <= 1 {
            write!(f, [space(), views.format()])?;
        } else {
            write!(f, [group(&soft_line_indent_or_space(&views.format()))])?;
        }

        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
