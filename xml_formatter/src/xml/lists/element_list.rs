use crate::prelude::*;
use xml_syntax::XmlElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlElementList;
impl FormatRule<XmlElementList> for FormatXmlElementList {
    type Context = XmlFormatContext;
    fn fmt(&self, node: &XmlElementList, f: &mut XmlFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}
