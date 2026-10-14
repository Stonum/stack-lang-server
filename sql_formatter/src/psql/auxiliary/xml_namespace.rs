use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlNamespace;
use sql_syntax::PsqlXmlNamespaceFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlNamespace;
impl FormatNodeRule<PsqlXmlNamespace> for FormatPsqlXmlNamespace {
    fn fmt_fields(&self, node: &PsqlXmlNamespace, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlNamespaceFields {
            uri,
            as_token,
            name,
        } = node.as_fields();

        write!(
            f,
            [
                uri.format(),
                space(),
                as_token.format(),
                space(),
                name.format()
            ]
        )
    }
}
