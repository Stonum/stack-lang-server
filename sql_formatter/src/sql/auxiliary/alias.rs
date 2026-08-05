use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAlias;
use sql_syntax::SqlAliasFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlias;
impl FormatNodeRule<SqlAlias> for FormatSqlAlias {
    fn fmt_fields(&self, node: &SqlAlias, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAliasFields {
            as_token,
            value,
            columns,
        } = node.as_fields();

        if let Some(as_token) = as_token {
            write!(f, [as_token.format(), space()])?;
        }
        write!(f, [value.format()])?;
        if let Some(columns) = columns {
            write!(f, [columns.format()])?;
        }
        Ok(())
    }
}
