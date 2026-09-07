use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlAttributeInitializerClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlAttributeInitializerClause;
impl FormatNodeRule<XmlAttributeInitializerClause> for FormatXmlAttributeInitializerClause {
    fn fmt_fields(
        &self,
        node: &XmlAttributeInitializerClause,
        f: &mut XmlFormatter,
    ) -> FormatResult<()> {
        write!(f, [node.eq_token().format(), node.value().format()])
    }
}
