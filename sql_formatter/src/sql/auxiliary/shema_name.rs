use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlShemaName;
use sql_syntax::SqlShemaNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlShemaName;
impl FormatNodeRule<SqlShemaName> for FormatSqlShemaName {
    fn fmt_fields(&self, node: &SqlShemaName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlShemaNameFields {
            base,
            name,
            dot_token,
        } = node.as_fields();

        if let Some(base) = base {
            write!(f, [base.format()])?;
        }
        write!(f, [name.format(), dot_token.format()])
    }
}
