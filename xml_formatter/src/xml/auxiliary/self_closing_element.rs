use crate::prelude::*;
use biome_formatter::{format_args, write};
use xml_syntax::XmlSelfClosingElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlSelfClosingElement;
impl FormatNodeRule<XmlSelfClosingElement> for FormatXmlSelfClosingElement {
    fn fmt_fields(&self, node: &XmlSelfClosingElement, f: &mut XmlFormatter) -> FormatResult<()> {
        let attributes = node.attributes();
        let loose = f.options().self_closing_spacing().is_loose();

        write!(f, [node.l_angle_token().format(), node.name().format()])?;

        match attributes.len() {
            0 => {}
            1 => write!(f, [space(), attributes.format()])?,
            _ => write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break_or_space(),
                    attributes.format()
                ]))]
            )?,
        }

        if loose {
            write!(f, [space()])?;
        }

        write!(
            f,
            [node.slash_token().format(), node.r_angle_token().format()]
        )
    }
}
