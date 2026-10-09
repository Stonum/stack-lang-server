use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlPrivilege;
use sql_syntax::SqlPrivilegeFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrivilege;
impl FormatNodeRule<SqlPrivilege> for FormatSqlPrivilege {
    fn fmt_fields(&self, node: &SqlPrivilege, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPrivilegeFields {
            kind,
            privileges_token,
            columns,
        } = node.as_fields();

        write!(f, [kind.format()])?;
        if let Some(privileges_token) = privileges_token {
            write!(f, [space(), privileges_token.format()])?;
        }
        if let Some(columns) = columns {
            write!(f, [space(), columns.format()])?;
        }
        Ok(())
    }
}
