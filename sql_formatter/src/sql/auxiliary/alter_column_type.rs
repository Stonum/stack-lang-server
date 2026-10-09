use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlAlterColumnType;
use sql_syntax::SqlAlterColumnTypeFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlAlterColumnType;
impl FormatNodeRule<SqlAlterColumnType> for FormatSqlAlterColumnType {
    fn fmt_fields(&self, node: &SqlAlterColumnType, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlAlterColumnTypeFields {
            set_token,
            data_token,
            type_token,
            ty,
            using,
        } = node.as_fields();

        for token in [&set_token, &data_token].into_iter().flatten() {
            write!(f, [token.format(), space()])?;
        }
        write!(f, [type_token.format(), space(), ty.format()])?;
        if let Some(using) = using {
            write!(f, [space(), using.format()])?;
        }
        Ok(())
    }
}
