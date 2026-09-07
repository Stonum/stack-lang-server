use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlName;
impl FormatNodeRule<XmlName> for FormatXmlName {
    fn fmt_fields(&self, node: &XmlName, f: &mut XmlFormatter) -> FormatResult<()> {
        write!(f, [node.value_token().format()])
    }
}
