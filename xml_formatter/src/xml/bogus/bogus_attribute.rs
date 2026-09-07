use crate::FormatBogusNodeRule;
use xml_syntax::XmlBogusAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlBogusAttribute;
impl FormatBogusNodeRule<XmlBogusAttribute> for FormatXmlBogusAttribute {}
