use crate::prelude::*;
use xml_syntax::XmlAttributeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlAttributeList;
impl FormatRule<XmlAttributeList> for FormatXmlAttributeList {
    type Context = XmlFormatContext;
    fn fmt(&self, node: &XmlAttributeList, f: &mut XmlFormatter) -> FormatResult<()> {
        // The enclosing tag rule wraps this in `group(indent(...))`, so the
        // separator collapses to a single space when the tag fits on one
        // line and to an indented line break otherwise.
        f.join_with(&soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
