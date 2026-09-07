use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{ParseDiagnostic, TokenSource};
use biome_parser::token_source::{
    BumpWithContext, TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia,
};
use biome_rowan::{TextRange, TextSize, TriviaPieceKind};

use crate::lexer::{XmlLexContext, XmlLexer};
use xml_syntax::XmlSyntaxKind::{self, EOF};

/// Token source for the parser that skips over any trivia token.
pub struct XmlTokenSource<'l> {
    lexer: BufferedLexer<XmlSyntaxKind, XmlLexer<'l>>,

    /// Skipped trivia. Needed to construct the CST and compute the
    /// non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

pub type XmlTokenSourceCheckpoint = TokenSourceCheckpoint<XmlSyntaxKind>;

impl<'l> XmlTokenSource<'l> {
    pub(crate) fn new(lexer: BufferedLexer<XmlSyntaxKind, XmlLexer<'l>>) -> XmlTokenSource<'l> {
        XmlTokenSource {
            lexer,
            trivia_list: vec![],
        }
    }

    pub fn from_str(source: &'l str) -> XmlTokenSource<'l> {
        let lexer = XmlLexer::from_str(source);
        let buffered = BufferedLexer::new(lexer);
        let mut source = XmlTokenSource::new(buffered);

        source.next_non_trivia_token(XmlLexContext::Regular, true);
        source
    }

    #[inline]
    fn next_non_trivia_token(&mut self, context: XmlLexContext, first_token: bool) {
        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(context);

            match TriviaPieceKind::try_from(kind) {
                Err(_) => break,
                Ok(trivia_kind) => {
                    // Trivia at and after a newline is the leading trivia of the next token.
                    if trivia_kind.is_newline() {
                        trailing = false;
                    }

                    self.trivia_list
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                }
            }
        }
    }

    pub fn checkpoint(&self) -> XmlTokenSourceCheckpoint {
        XmlTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: XmlTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }
}

impl<'source> TokenSource for XmlTokenSource<'source> {
    type Kind = XmlSyntaxKind;

    #[inline(always)]
    fn current(&self) -> XmlSyntaxKind {
        self.lexer.current()
    }

    #[inline(always)]
    fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    #[inline(always)]
    fn text(&self) -> &'source str {
        self.lexer.source()
    }

    #[inline(always)]
    fn position(&self) -> TextSize {
        self.current_range().start()
    }

    #[inline(always)]
    fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.bump_with_context(XmlLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(XmlLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for XmlTokenSource<'_> {
    type Context = XmlLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(context, false);
        }
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(context, true)
        }
    }
}

impl<'source> TokenSourceWithBufferedLexer<XmlLexer<'source>> for XmlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<XmlSyntaxKind, XmlLexer<'source>> {
        &mut self.lexer
    }
}
