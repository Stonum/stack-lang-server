use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlShemaName;
use psql_syntax::PsqlShemaNameFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlShemaName;
impl FormatNodeRule<PsqlShemaName> for FormatPsqlShemaName {
    fn fmt_fields(&self, node: &PsqlShemaName, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlShemaNameFields {
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
