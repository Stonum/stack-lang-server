use crate::prelude::*;
use crate::rules::verbatim_attributes::write_verbatim_attributes;
use biome_formatter::{format_args, write};
use xml_syntax::XmlOpeningElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlOpeningElement;
impl FormatNodeRule<XmlOpeningElement> for FormatXmlOpeningElement {
    fn fmt_fields(&self, node: &XmlOpeningElement, f: &mut XmlFormatter) -> FormatResult<()> {
        let attributes = node.attributes();

        write!(f, [node.l_angle_token().format(), node.name().format()])?;

        if attributes.len() > 0 && f.options().verbatim_attributes() {
            // Reproduce the attributes as authored; only the closing `>` is
            // repositioned -- onto its own line when the attributes wrap,
            // otherwise left where the last attribute ends.
            let multiline = write_verbatim_attributes(node.name().ok(), &attributes, f)?;
            if multiline {
                write!(f, [hard_line_break()])?;
            }
            return write!(f, [node.r_angle_token().format()]);
        }

        match attributes.len() {
            0 => {}
            // A single attribute never benefits from wrapping (even a huge
            // multi-line value stays attached to the tag name).
            1 => write!(f, [space(), attributes.format()])?,
            // When the attributes wrap, `>` drops to its own line, aligned
            // with the tag name.
            _ => write!(
                f,
                [group(&format_args![
                    indent(&format_args![
                        soft_line_break_or_space(),
                        attributes.format()
                    ]),
                    soft_line_break(),
                ])]
            )?,
        }

        write!(f, [node.r_angle_token().format()])
    }
}
