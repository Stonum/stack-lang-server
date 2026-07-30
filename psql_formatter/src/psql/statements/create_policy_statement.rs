use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCreatePolicyStatement;
use psql_syntax::PsqlCreatePolicyStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreatePolicyStatement;
impl FormatNodeRule<PsqlCreatePolicyStatement> for FormatPsqlCreatePolicyStatement {
    fn fmt_fields(
        &self,
        node: &PsqlCreatePolicyStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreatePolicyStatementFields {
            create_token,
            policy_token,
            name,
            on_token,
            table,
            for_clause,
            using_clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                create_token.format(),
                space(),
                policy_token.format(),
                space(),
                name.format(),
                space(),
                on_token.format(),
                space(),
                table.format(),
            ]
        )?;

        if let Some(for_clause) = for_clause {
            write!(f, [space(), for_clause.format()])?;
        }
        if let Some(using_clause) = using_clause {
            write!(f, [space(), using_clause.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
