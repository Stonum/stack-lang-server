use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlProcessingInstruction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlProcessingInstruction;
impl FormatNodeRule<XmlProcessingInstruction> for FormatXmlProcessingInstruction {
    fn fmt_fields(
        &self,
        node: &XmlProcessingInstruction,
        f: &mut XmlFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [
                node.l_angle_question_token().format(),
                node.target().format(),
            ]
        )?;
        if let Some(content) = node.content_token() {
            write!(f, [content.format()])?;
        }
        write!(f, [node.question_r_angle_token().format()])
    }
}
