use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlRoot;
impl FormatNodeRule<XmlRoot> for FormatXmlRoot {
    fn fmt_fields(&self, node: &XmlRoot, f: &mut XmlFormatter) -> FormatResult<()> {
        let has_prolog = node.prolog().is_some();
        let content = node.content();
        let has_content = content.len() > 0;

        if let Some(bom) = node.bom_token() {
            write!(f, [bom.format()])?;
        }

        if let Some(prolog) = node.prolog() {
            write!(f, [prolog.format()])?;
        }

        if has_content {
            if has_prolog {
                write!(f, [hard_line_break()])?;
            }
            write!(f, [content.format()])?;
        }

        if let Ok(eof) = node.eof_token() {
            write!(f, [eof.format()])?;
        }

        if has_prolog || has_content {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}
