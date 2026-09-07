use crate::prelude::*;
use biome_formatter::{format_args, write};
use xml_syntax::XmlOpeningElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlOpeningElement;
impl FormatNodeRule<XmlOpeningElement> for FormatXmlOpeningElement {
    fn fmt_fields(&self, node: &XmlOpeningElement, f: &mut XmlFormatter) -> FormatResult<()> {
        let attributes = node.attributes();

        write!(f, [node.l_angle_token().format(), node.name().format()])?;

        match attributes.len() {
            0 => {}
            // A single attribute never benefits from wrapping (even a huge
            // multi-line value stays attached to the tag name).
            1 => write!(f, [space(), attributes.format()])?,
            _ => write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break_or_space(),
                    attributes.format()
                ]))]
            )?,
        }

        write!(f, [node.r_angle_token().format()])
    }
}
