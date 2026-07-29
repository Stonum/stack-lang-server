use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDropTableStatement;
use psql_syntax::PsqlDropTableStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropTableStatement;
impl FormatNodeRule<PsqlDropTableStatement> for FormatPsqlDropTableStatement {
    fn fmt_fields(&self, node: &PsqlDropTableStatement, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlDropTableStatementFields {
            drop_token,
            table_token,
            if_token,
            exists_token,
            tables,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), table_token.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(exists_token) = exists_token {
            write!(f, [space(), exists_token.format()])?;
        }

        if tables.len() <= 1 {
            write!(f, [space(), tables.format()])?;
        } else {
            write!(f, [group(&soft_line_indent_or_space(&tables.format()))])?;
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
