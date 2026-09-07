//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use xml_syntax::AnyXmlAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyXmlAttribute;
impl FormatRule<AnyXmlAttribute> for FormatAnyXmlAttribute {
    type Context = XmlFormatContext;
    fn fmt(&self, node: &AnyXmlAttribute, f: &mut XmlFormatter) -> FormatResult<()> {
        match node {
            AnyXmlAttribute::XmlAttribute(node) => node.format().fmt(f),
            AnyXmlAttribute::XmlBogusAttribute(node) => node.format().fmt(f),
        }
    }
}
