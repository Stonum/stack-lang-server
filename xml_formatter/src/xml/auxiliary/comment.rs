use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlComment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlComment;
impl FormatNodeRule<XmlComment> for FormatXmlComment {
    fn fmt_fields(&self, node: &XmlComment, f: &mut XmlFormatter) -> FormatResult<()> {
        write!(f, [node.comment_start_token().format()])?;
        if let Some(content) = node.content_token() {
            write!(f, [content.format()])?;
        }
        write!(f, [node.comment_end_token().format()])
    }
}
