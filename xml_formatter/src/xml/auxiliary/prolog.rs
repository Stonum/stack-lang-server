use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlProlog;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlProlog;
impl FormatNodeRule<XmlProlog> for FormatXmlProlog {
    fn fmt_fields(&self, node: &XmlProlog, f: &mut XmlFormatter) -> FormatResult<()> {
        let attributes = node.attributes();

        write!(f, [node.xml_decl_start_token().format()])?;

        if attributes.len() > 0 {
            // Keep the declaration on one line: its own group so the shared
            // attribute-list separator collapses to a single space.
            write!(f, [space(), group(&attributes.format())])?;
        }

        write!(f, [node.question_r_angle_token().format()])
    }
}
