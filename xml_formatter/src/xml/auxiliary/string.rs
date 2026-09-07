use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlString;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlString;
impl FormatNodeRule<XmlString> for FormatXmlString {
    fn fmt_fields(&self, node: &XmlString, f: &mut XmlFormatter) -> FormatResult<()> {
        // Verbatim: quote style and entity references (`&apos;`, `&quot;`,
        // …) inside the value are load-bearing and must not be rewritten.
        write!(f, [node.value_token().format()])
    }
}
