use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlAttribute;
impl FormatNodeRule<XmlAttribute> for FormatXmlAttribute {
    fn fmt_fields(&self, node: &XmlAttribute, f: &mut XmlFormatter) -> FormatResult<()> {
        write!(f, [node.name().format(), node.initializer().format()])
    }
}
