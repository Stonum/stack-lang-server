use crate::prelude::*;
use xml_syntax::XmlSyntaxToken;

use biome_formatter::trivia::format_skipped_token_trivia;
use biome_formatter::write;

use crate::XmlFormatContext;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatXmlSyntaxToken;
impl_format!(XmlSyntaxToken, FormatXmlSyntaxToken);

impl FormatRule<XmlSyntaxToken> for FormatXmlSyntaxToken {
    type Context = XmlFormatContext;

    fn fmt(&self, token: &XmlSyntaxToken, f: &mut XmlFormatter) -> FormatResult<()> {
        f.state_mut().track_token(token);

        write!(f, [format_skipped_token_trivia(token)])?;

        let text = token.text_trimmed();
        if text.contains('\r') {
            // A multi-line token here is only ever an attribute string
            // literal (e.g. a value carrying an embedded SQL fragment across
            // several lines). `biome_formatter`'s text builder rejects a raw
            // `\r` in printed content, so normalize the embedded line ending
            // to `\n`; XML doesn't care about the line-ending style inside a
            // quoted value.
            let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
            let start = token.text_trimmed_range().start();
            return write!(f, [dynamic_text(&normalized, start)]);
        }

        write!(f, [format_trimmed_token(token)])
    }
}
