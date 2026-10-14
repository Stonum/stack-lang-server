use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlTableValueColumn;
use sql_syntax::PsqlXmlTableValueColumnFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlTableValueColumn;
impl FormatNodeRule<PsqlXmlTableValueColumn> for FormatPsqlXmlTableValueColumn {
    fn fmt_fields(&self, node: &PsqlXmlTableValueColumn, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlTableValueColumnFields {
            name,
            ty,
            path,
            default,
            not_null,
        } = node.as_fields();

        write!(f, [name.format(), space(), ty.format()])?;
        if let Some(path) = path {
            write!(f, [space(), path.format()])?;
        }
        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }
        if let Some(not_null) = not_null {
            write!(f, [space(), not_null.format()])?;
        }
        Ok(())
    }
}
