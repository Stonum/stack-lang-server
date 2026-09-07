use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlClosingElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlClosingElement;
impl FormatNodeRule<XmlClosingElement> for FormatXmlClosingElement {
    fn fmt_fields(&self, node: &XmlClosingElement, f: &mut XmlFormatter) -> FormatResult<()> {
        write!(
            f,
            [
                node.l_angle_token().format(),
                node.slash_token().format(),
                node.name().format(),
                node.r_angle_token().format(),
            ]
        )
    }
}
