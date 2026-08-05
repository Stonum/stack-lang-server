use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlCteDefinition;
use sql_syntax::SqlCteDefinitionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCteDefinition;
impl FormatNodeRule<SqlCteDefinition> for FormatSqlCteDefinition {
    fn fmt_fields(&self, node: &SqlCteDefinition, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCteDefinitionFields {
            name,
            columns,
            as_token,
            materialized,
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(columns) = columns {
            write!(f, [columns.format()])?;
        }
        write!(f, [space(), as_token.format()])?;
        if let Some(materialized) = materialized {
            write!(f, [space(), materialized.format()])?;
        }
        write!(
            f,
            [
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&query.format())),
                r_paren_token.format(),
            ]
        )
    }
}
