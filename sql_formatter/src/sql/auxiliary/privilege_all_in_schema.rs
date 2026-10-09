use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::SqlPrivilegeAllInSchema;
use sql_syntax::SqlPrivilegeAllInSchemaFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlPrivilegeAllInSchema;
impl FormatNodeRule<SqlPrivilegeAllInSchema> for FormatSqlPrivilegeAllInSchema {
    fn fmt_fields(&self, node: &SqlPrivilegeAllInSchema, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlPrivilegeAllInSchemaFields {
            on_token,
            all_token,
            object_kind,
            in_token,
            schema_token,
            schemas,
        } = node.as_fields();

        let keyword = format_with(|f| {
            write!(
                f,
                [
                    on_token.format(),
                    space(),
                    all_token.format(),
                    space(),
                    object_kind.format(),
                    space(),
                    in_token.format(),
                    space(),
                    schema_token.format()
                ]
            )
        });
        write_wrapping_fill_clause(keyword, &schemas, |_| false, f)
    }
}
