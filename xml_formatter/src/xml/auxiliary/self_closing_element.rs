use crate::prelude::*;
use crate::rules::verbatim_attributes::write_verbatim_attributes;
use biome_formatter::{format_args, write};
use xml_syntax::XmlSelfClosingElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlSelfClosingElement;
impl FormatNodeRule<XmlSelfClosingElement> for FormatXmlSelfClosingElement {
    fn fmt_fields(&self, node: &XmlSelfClosingElement, f: &mut XmlFormatter) -> FormatResult<()> {
        let attributes = node.attributes();
        let loose = f.options().self_closing_spacing().is_loose();

        write!(f, [node.l_angle_token().format(), node.name().format()])?;

        if attributes.len() > 0 && f.options().verbatim_attributes() {
            // Reproduce the attributes as authored; only `/>` is repositioned
            // -- onto its own line when the attributes wrap, otherwise left
            // where the last attribute ends (with the configured spacing).
            let multiline = write_verbatim_attributes(node.name().ok(), &attributes, f)?;
            if multiline {
                write!(f, [hard_line_break()])?;
            } else if loose {
                write!(f, [space()])?;
            }
            return write!(
                f,
                [node.slash_token().format(), node.r_angle_token().format()]
            );
        }

        match attributes.len() {
            0 | 1 => {
                if attributes.len() == 1 {
                    write!(f, [space(), attributes.format()])?;
                }
                if loose {
                    write!(f, [space()])?;
                }
            }
            // When the attributes wrap, `/>` drops to its own line, aligned
            // with the tag name; when they fit, a space before `/>` iff the
            // spacing option is `Loose`.
            _ if loose => write!(
                f,
                [group(&format_args![
                    indent(&format_args![
                        soft_line_break_or_space(),
                        attributes.format()
                    ]),
                    soft_line_break_or_space(),
                ])]
            )?,
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

        write!(
            f,
            [node.slash_token().format(), node.r_angle_token().format()]
        )
    }
}
