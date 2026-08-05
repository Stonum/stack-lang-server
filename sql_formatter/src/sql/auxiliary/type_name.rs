use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTypeName;
use sql_syntax::SqlTypeNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTypeName;
impl FormatNodeRule<SqlTypeName> for FormatSqlTypeName {
    fn fmt_fields(&self, node: &SqlTypeName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTypeNameFields {
            name,
            args,
            modifier,
            array_suffix,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(args) = args {
            write!(f, [args.format()])?;
        }
        if let Some(modifier) = modifier {
            write!(f, [space(), modifier.format()])?;
        }
        if let Some(array_suffix) = array_suffix {
            write!(f, [array_suffix.format()])?;
        }
        Ok(())
    }
}
