use crate::FormatBogusNodeRule;
use xml_syntax::XmlBogusElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlBogusElement;
impl FormatBogusNodeRule<XmlBogusElement> for FormatXmlBogusElement {}
