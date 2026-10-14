use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlColumnDefault;
use sql_syntax::PsqlXmlColumnDefaultFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlColumnDefault;
impl FormatNodeRule<PsqlXmlColumnDefault> for FormatPsqlXmlColumnDefault {
    fn fmt_fields(&self, node: &PsqlXmlColumnDefault, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlColumnDefaultFields {
            default_token,
            value,
        } = node.as_fields();

        write!(f, [default_token.format(), space(), value.format()])
    }
}
