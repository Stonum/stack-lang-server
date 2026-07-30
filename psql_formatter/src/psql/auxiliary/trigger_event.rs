use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTriggerEvent;
use psql_syntax::PsqlTriggerEventFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTriggerEvent;
impl FormatNodeRule<PsqlTriggerEvent> for FormatPsqlTriggerEvent {
    fn fmt_fields(&self, node: &PsqlTriggerEvent, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTriggerEventFields {
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
