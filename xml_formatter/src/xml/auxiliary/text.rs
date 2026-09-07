use std::borrow::Cow;

use crate::prelude::*;
use biome_formatter::write;
use xml_syntax::XmlText;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlText;
impl FormatNodeRule<XmlText> for FormatXmlText {
    fn fmt_fields(&self, node: &XmlText, f: &mut XmlFormatter) -> FormatResult<()> {
        let token = node.value_token()?;
        let raw = token.text_trimmed();
        // The lexer folds every byte up to the next `<` into the text token,
        // so it can carry trailing indentation/newlines. `.rx`/`.xdic` have
        // no significant mixed content, so trim that off and normalize any
        // embedded CR the printer would reject.
        let normalized = raw.replace("\r\n", "\n").replace('\r', "\n");
        let trimmed = normalized.trim_end();

        if trimmed == raw {
            return write!(f, [token.format()]);
        }

        write!(
            f,
            [format_replaced(
                &token,
                &syntax_token_cow_slice(
                    Cow::Owned(trimmed.to_owned()),
                    &token,
                    token.text_trimmed_range().start(),
                )
            )]
        )
    }
}
