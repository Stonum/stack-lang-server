use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCteDefinition;
use psql_syntax::PsqlCteDefinitionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteDefinition;
impl FormatNodeRule<PsqlCteDefinition> for FormatPsqlCteDefinition {
    fn fmt_fields(&self, node: &PsqlCteDefinition, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlCteDefinitionFields {
            name,
            columns,
            as_token,
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(columns) = columns {
            write!(f, [columns.format()])?;
        }
        write!(
            f,
            [
                space(),
                as_token.format(),
                space(),
                l_paren_token.format(),
                block_indent(&query.format()),
                r_paren_token.format(),
            ]
        )
    }
}
