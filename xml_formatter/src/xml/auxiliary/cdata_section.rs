use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlCdataSection;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlCdataSection;
impl FormatNodeRule<XmlCdataSection> for FormatXmlCdataSection {
    fn fmt_fields(&self, node: &XmlCdataSection, f: &mut XmlFormatter) -> FormatResult<()> {
        write!(f, [node.cdata_start_token().format()])?;
        if let Some(content) = node.content_token() {
            write!(f, [content.format()])?;
        }
        write!(f, [node.cdata_end_token().format()])
    }
}
