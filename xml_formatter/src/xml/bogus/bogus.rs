use crate::FormatBogusNodeRule;
use xml_syntax::XmlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlBogus;
impl FormatBogusNodeRule<XmlBogus> for FormatXmlBogus {}
