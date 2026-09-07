mod tests;

use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, TokenFlags};
use biome_rowan::SyntaxKind;
use xml_syntax::XmlSyntaxKind::{
    self, CDATA_END, CDATA_START, COMMENT_END, COMMENT_START, EOF, ERROR_TOKEN, L_ANGLE_QUESTION,
    NEWLINE, QUESTION_R_ANGLE, TOMBSTONE, UNICODE_BOM, WHITESPACE, XML_DECL_START, XML_LITERAL,
    XML_STRING_LITERAL, XML_TEXT,
};
use xml_syntax::{T, TextRange, TextSize};

/// The lexing context the parser puts the lexer in for the next token.
///
/// XML is context-sensitive at the lexical level: the same bytes lex
/// differently inside a tag, between tags, and inside a comment / CDATA /
/// processing-instruction body.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum XmlLexContext {
    /// Tag interiors, tag starts and the XML-declaration interior: angle
    /// brackets, names, `=`, quoted strings.
    #[default]
    Regular,
    /// Element content between tags: free text plus the starts of nested
    /// tags / comments / CDATA sections / processing instructions.
    ElementList,
    /// A whole `<!-- ... -->` comment.
    Comment,
    /// A whole `<![CDATA[ ... ]]>` section.
    Cdata,
    /// The body of a generic `<?target ... ?>` processing instruction,
    /// everything up to and including the closing `?>`.
    PiContent,
}

impl LexContext for XmlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, XmlLexContext::Regular)
    }
}

#[derive(Debug)]
pub(crate) struct XmlLexer<'src> {
    source: &'src str,
    position: usize,
    current_kind: XmlSyntaxKind,
    current_start: TextSize,
    current_flags: TokenFlags,
    after_newline: bool,
    unicode_bom_length: usize,
    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> XmlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            after_newline: false,
            unicode_bom_length: 0,
            diagnostics: vec![],
        }
    }

    #[inline]
    fn consume_byte(&mut self, kind: XmlSyntaxKind) -> XmlSyntaxKind {
        self.advance(1);
        kind
    }

    fn try_consume_bom(&mut self) -> bool {
        match self.consume_potential_bom(UNICODE_BOM) {
            Some((_, bom_size)) => {
                self.unicode_bom_length = bom_size;
                true
            }
            None => false,
        }
    }

    /// Whether the source at the current position starts with `needle`.
    fn at_str(&self, needle: &str) -> bool {
        self.source.as_bytes()[self.position..].starts_with(needle.as_bytes())
    }

    /// Whether the current position starts an XML declaration: `<?xml`
    /// followed by whitespace, the closing `?>`, or end of input.
    fn at_xml_decl(&self) -> bool {
        self.at_str("<?xml")
            && match self.byte_at(5) {
                Some(b) => b.is_ascii_whitespace() || b == b'?',
                None => true,
            }
    }

    // --- context handlers -------------------------------------------------

    fn consume_regular(&mut self, current: u8) -> XmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' => self.consume_l_angle(),
            b'>' => self.consume_byte(T![>]),
            b'/' => self.consume_byte(T![/]),
            b'=' => self.consume_byte(T![=]),
            b'?' if self.peek_byte() == Some(b'>') => {
                self.advance(2);
                QUESTION_R_ANGLE
            }
            b'\'' | b'"' => self.consume_string_literal(current),
            _ if is_xml_name_start_byte(current) => self.consume_name(),
            _ => {
                if self.position == 0 && self.try_consume_bom() {
                    return UNICODE_BOM;
                }
                if !current.is_ascii() && is_xml_name_start_char(self.current_char_unchecked()) {
                    return self.consume_name();
                }
                self.consume_unexpected_character()
            }
        }
    }

    fn consume_element_list(&mut self, current: u8) -> XmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' => self.consume_l_angle(),
            _ => {
                if self.position == 0 && self.try_consume_bom() {
                    return UNICODE_BOM;
                }
                self.consume_text()
            }
        }
    }

    fn consume_comment(&mut self, current: u8) -> XmlSyntaxKind {
        match current {
            b'<' if self.at_str("<!--") => {
                self.advance(4);
                COMMENT_START
            }
            b'-' if self.at_str("-->") => {
                self.advance(3);
                COMMENT_END
            }
            _ => {
                while let Some(byte) = self.current_byte() {
                    if byte == b'-' && self.at_str("-->") {
                        break;
                    }
                    self.advance_byte_or_char(byte);
                }
                XML_LITERAL
            }
        }
    }

    fn consume_cdata(&mut self, current: u8) -> XmlSyntaxKind {
        match current {
            b'<' if self.at_str("<![CDATA[") => {
                self.advance(9);
                CDATA_START
            }
            b']' if self.at_str("]]>") => {
                self.advance(3);
                CDATA_END
            }
            _ => {
                while let Some(byte) = self.current_byte() {
                    if byte == b']' && self.at_str("]]>") {
                        break;
                    }
                    self.advance_byte_or_char(byte);
                }
                XML_LITERAL
            }
        }
    }

    fn consume_pi_content(&mut self, current: u8) -> XmlSyntaxKind {
        if current == b'?' && self.peek_byte() == Some(b'>') {
            self.advance(2);
            return QUESTION_R_ANGLE;
        }

        while let Some(byte) = self.current_byte() {
            if byte == b'?' && self.peek_byte() == Some(b'>') {
                break;
            }
            self.advance_byte_or_char(byte);
        }
        XML_LITERAL
    }

    fn consume_l_angle(&mut self) -> XmlSyntaxKind {
        self.assert_byte(b'<');

        if self.at_str("<!--") {
            self.advance(4);
            COMMENT_START
        } else if self.at_str("<![CDATA[") {
            self.advance(9);
            CDATA_START
        } else if self.at_xml_decl() {
            self.advance(5);
            XML_DECL_START
        } else if self.at_str("<?") {
            self.advance(2);
            L_ANGLE_QUESTION
        } else {
            self.consume_byte(T![<])
        }
    }

    fn consume_name(&mut self) -> XmlSyntaxKind {
        while let Some(byte) = self.current_byte() {
            if byte.is_ascii() {
                if is_xml_name_continue_byte(byte) {
                    self.advance(1);
                } else {
                    break;
                }
            } else {
                let chr = self.current_char_unchecked();
                if is_xml_name_continue_char(chr) {
                    self.advance_char_unchecked();
                } else {
                    break;
                }
            }
        }
        XML_LITERAL
    }

    fn consume_text(&mut self) -> XmlSyntaxKind {
        while let Some(byte) = self.current_byte() {
            if byte == b'<' {
                break;
            }
            self.advance_byte_or_char(byte);
        }
        XML_TEXT
    }

    fn consume_string_literal(&mut self, quote: u8) -> XmlSyntaxKind {
        let start = self.current_start;
        self.advance(1); // opening quote

        while let Some(byte) = self.current_byte() {
            if byte == quote {
                self.advance(1);
                return XML_STRING_LITERAL;
            }
            self.advance_byte_or_char(byte);
        }

        self.push_diagnostic(ParseDiagnostic::new(
            "Missing closing quote",
            start..self.text_position(),
        ));
        ERROR_TOKEN
    }

    fn consume_unexpected_character(&mut self) -> XmlSyntaxKind {
        self.assert_current_char_boundary();
        let chr = self.current_char_unchecked();
        let start = self.text_position();
        self.advance(chr.len_utf8());
        self.push_diagnostic(ParseDiagnostic::new(
            format!("Unexpected character `{chr}`"),
            start..self.text_position(),
        ));
        ERROR_TOKEN
    }
}

impl<'src> Lexer<'src> for XmlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = XmlSyntaxKind;
    type LexContext = XmlLexContext;
    type ReLexContext = ();

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() {
            EOF
        } else {
            let current = self.current_byte().unwrap();
            match context {
                XmlLexContext::Regular => self.consume_regular(current),
                XmlLexContext::ElementList => self.consume_element_list(current),
                XmlLexContext::Comment => self.consume_comment(current),
                XmlLexContext::Cdata => self.consume_cdata(current),
                XmlLexContext::PiContent => self.consume_pi_content(current),
            }
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
        } else if kind == NEWLINE {
            self.after_newline = true;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            after_line_break,
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        self.position = u32::from(position) as usize;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.current_kind = current_kind;
        self.after_newline = after_line_break;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn position(&self) -> usize {
        self.position
    }

    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

impl<'src> LexerWithCheckpoint<'src> for XmlLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

/// ASCII bytes that may start an XML name (`NameStartChar`, ASCII subset).
fn is_xml_name_start_byte(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'_' | b':')
}

/// ASCII bytes that may continue an XML name (`NameChar`, ASCII subset).
fn is_xml_name_continue_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b':' | b'-' | b'.')
}

/// Non-ASCII characters that may start an XML name. Approximates the XML
/// `NameStartChar` production with `char::is_alphabetic`, which covers the
/// Cyrillic names ubiquitous in Stack resources and dictionaries.
fn is_xml_name_start_char(chr: char) -> bool {
    chr.is_alphabetic()
}

fn is_xml_name_continue_char(chr: char) -> bool {
    chr.is_alphanumeric()
}
