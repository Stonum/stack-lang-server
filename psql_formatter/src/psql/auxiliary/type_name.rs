use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTypeName;
use psql_syntax::PsqlTypeNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTypeName;
impl FormatNodeRule<PsqlTypeName> for FormatPsqlTypeName {
    fn fmt_fields(&self, node: &PsqlTypeName, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTypeNameFields {
            name,
            args,
            array_suffix,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(args) = args {
            write!(f, [args.format()])?;
        }
        if let Some(array_suffix) = array_suffix {
            write!(f, [array_suffix.format()])?;
        }
        Ok(())
    }
}
