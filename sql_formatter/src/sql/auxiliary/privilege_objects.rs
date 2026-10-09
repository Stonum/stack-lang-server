use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::SqlPrivilegeObjects;
use sql_syntax::SqlPrivilegeObjectsFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrivilegeObjects;
impl FormatNodeRule<SqlPrivilegeObjects> for FormatSqlPrivilegeObjects {
    fn fmt_fields(&self, node: &SqlPrivilegeObjects, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPrivilegeObjectsFields {
            on_token,
            object_kind,
            objects,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(f, [on_token.format()])?;
            if let Some(object_kind) = &object_kind {
                write!(f, [space(), object_kind.format()])?;
            }
            Ok(())
        });
        write_wrapping_fill_clause(keyword, &objects, |_| false, f)
    }
}
