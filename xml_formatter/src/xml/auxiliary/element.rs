use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::{AnyXmlElement, XmlElement};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlElement;
impl FormatNodeRule<XmlElement> for FormatXmlElement {
    fn fmt_fields(&self, node: &XmlElement, f: &mut XmlFormatter) -> FormatResult<()> {
        let children = node.children();

        write!(f, [node.opening().format()])?;

        if children.len() > 0 {
            // Content that is only text / CDATA stays inline (`<a>value</a>`);
            // anything with nested elements, comments or PIs gets each child
            // on its own indented line.
            let inline = children.iter().all(|child| {
                matches!(
                    child,
                    AnyXmlElement::XmlText(_) | AnyXmlElement::XmlCdataSection(_)
                )
            });

            if inline {
                write!(f, [children.format()])?;
            } else {
                write!(f, [block_indent(&children.format())])?;
            }
        }

        write!(f, [node.closing().format()])
    }
}
