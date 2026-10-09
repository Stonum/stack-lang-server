use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCreateMaterializedViewStatement;
use sql_syntax::PsqlCreateMaterializedViewStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreateMaterializedViewStatement;
impl FormatNodeRule<PsqlCreateMaterializedViewStatement>
    for FormatPsqlCreateMaterializedViewStatement
{
    fn fmt_fields(
        &self,
        node: &PsqlCreateMaterializedViewStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreateMaterializedViewStatementFields {
            create_token,
            materialized_token,
            view_token,
            if_token,
            not_token,
            exists_token,
            name,
            columns,
            options,
            as_token,
            query,
            with_data,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                create_token.format(),
                space(),
                materialized_token.format(),
                space(),
                view_token.format()
            ]
        )?;
        for token in [&if_token, &not_token, &exists_token].into_iter().flatten() {
            write!(f, [space(), token.format()])?;
        }
        write!(f, [space(), name.format()])?;
        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }
        if let Some(options) = options {
            write!(f, [space(), options.format()])?;
        }

        write!(f, [space(), as_token.format(), space(), query.format()])?;

        if let Some(with_data) = with_data {
            write!(f, [space(), with_data.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
