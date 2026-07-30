use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDropTriggerStatement;
use psql_syntax::PsqlDropTriggerStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropTriggerStatement;
impl FormatNodeRule<PsqlDropTriggerStatement> for FormatPsqlDropTriggerStatement {
    fn fmt_fields(
        &self,
        node: &PsqlDropTriggerStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDropTriggerStatementFields {
            drop_token,
            trigger_token,
            if_token,
            exists_token,
            name,
            on_token,
            table,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), trigger_token.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(exists_token) = exists_token {
            write!(f, [space(), exists_token.format()])?;
        }

        write!(
            f,
            [
                space(),
                name.format(),
                space(),
                on_token.format(),
                space(),
                table.format(),
            ]
        )?;

        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
