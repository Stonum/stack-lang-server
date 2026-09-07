//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use xml_syntax::AnyXmlElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyXmlElement;
impl FormatRule<AnyXmlElement> for FormatAnyXmlElement {
    type Context = XmlFormatContext;
    fn fmt(&self, node: &AnyXmlElement, f: &mut XmlFormatter) -> FormatResult<()> {
        match node {
            AnyXmlElement::XmlBogusElement(node) => node.format().fmt(f),
            AnyXmlElement::XmlCdataSection(node) => node.format().fmt(f),
            AnyXmlElement::XmlComment(node) => node.format().fmt(f),
            AnyXmlElement::XmlElement(node) => node.format().fmt(f),
            AnyXmlElement::XmlProcessingInstruction(node) => node.format().fmt(f),
            AnyXmlElement::XmlSelfClosingElement(node) => node.format().fmt(f),
            AnyXmlElement::XmlText(node) => node.format().fmt(f),
        }
    }
}
