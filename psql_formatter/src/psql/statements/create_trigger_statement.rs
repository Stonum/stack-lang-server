use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCreateTriggerStatement;
use psql_syntax::PsqlCreateTriggerStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreateTriggerStatement;
impl FormatNodeRule<PsqlCreateTriggerStatement> for FormatPsqlCreateTriggerStatement {
    fn fmt_fields(
        &self,
        node: &PsqlCreateTriggerStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreateTriggerStatementFields {
            create_token,
            trigger_token,
            name,
            timing,
            events,
            on_token,
            table,
            referencing_clause,
            for_each_clause,
            execute_token,
            function_kind,
            function,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                create_token.format(),
                space(),
                trigger_token.format(),
                space(),
                name.format(),
                space(),
                timing.format(),
                space(),
                group(&events.format()),
                space(),
                on_token.format(),
                space(),
                table.format(),
            ]
        )?;

        if let Some(referencing_clause) = referencing_clause {
            write!(f, [space(), referencing_clause.format()])?;
        }
        if let Some(for_each_clause) = for_each_clause {
            write!(f, [space(), for_each_clause.format()])?;
        }

        write!(
            f,
            [
                space(),
                execute_token.format(),
                space(),
                function_kind.format(),
                space(),
                function.format(),
            ]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
