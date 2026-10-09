use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlRefreshMaterializedViewStatement;
use sql_syntax::PsqlRefreshMaterializedViewStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlRefreshMaterializedViewStatement;
impl FormatNodeRule<PsqlRefreshMaterializedViewStatement>
    for FormatPsqlRefreshMaterializedViewStatement
{
    fn fmt_fields(
        &self,
        node: &PsqlRefreshMaterializedViewStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlRefreshMaterializedViewStatementFields {
            refresh_token,
            materialized_token,
            view_token,
            concurrently_token,
            name,
            with_data,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                refresh_token.format(),
                space(),
                materialized_token.format(),
                space(),
                view_token.format()
            ]
        )?;
        if let Some(concurrently_token) = concurrently_token {
            write!(f, [space(), concurrently_token.format()])?;
        }
        write!(f, [space(), name.format()])?;
        if let Some(with_data) = with_data {
            write!(f, [space(), with_data.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
