use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTriggerEvent;
use sql_syntax::SqlTriggerEventFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTriggerEvent;
impl FormatNodeRule<SqlTriggerEvent> for FormatSqlTriggerEvent {
    fn fmt_fields(&self, node: &SqlTriggerEvent, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTriggerEventFields {
            or_token,
            kind,
            of_clause,
        } = node.as_fields();

        if let Some(or_token) = or_token {
            write!(f, [or_token.format(), space()])?;
        }
        write!(f, [kind.format()])?;

        if let Some(of_clause) = of_clause {
            write!(f, [space(), of_clause.format()])?;
        }
        Ok(())
    }
}
