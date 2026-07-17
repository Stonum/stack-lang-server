mod tests;

use psql_syntax::PsqlDialect;
use psql_syntax::PsqlSyntaxKind;
use psql_syntax::PsqlSyntaxKind::*;
use psql_syntax::T;

use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::SyntaxKind;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_unicode_table::{
    Dispatch::{self, *},
    lookup_byte,
};

/// Контекст лексера для PSQL
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PsqlLexContext;

impl LexContext for PsqlLexContext {
    fn is_regular(&self) -> bool {
        true
    }
}

/// Контекст повторного лексирования
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PsqlReLexContext {
    /// Reinterprets the current `~` operator token as the start of a
    /// mlang-dialect `~name~`/`~$name~` tilde name instead. Requested by the
    /// parser only at the few grammar points where a table/function name is
    /// expected -- a plain `~`/`~*`/`~~`/`!~` operator only ever appears
    /// *between* two operands, never at the start of a name, so the default
    /// lexing (operator) is always safe elsewhere.
    TildeName,
}

/// Лексер для PostgreSQL SQL
#[derive(Debug)]
pub struct PsqlLexer<'src> {
    source: &'src str,
    position: usize,
    after_newline: bool,
    unicode_bom_length: usize,
    current_start: TextSize,
    current_kind: PsqlSyntaxKind,
    current_flags: TokenFlags,
    diagnostics: Vec<ParseDiagnostic>,
    dialect: PsqlDialect,
}

impl<'src> Lexer<'src> for PsqlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = PsqlSyntaxKind;
    type LexContext = PsqlLexContext;
    type ReLexContext = PsqlReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() { EOF } else { self.lex_token() };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
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

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn consume_newline_or_whitespaces(&mut self) -> PsqlSyntaxKind {
        if self.consume_newline() {
            self.after_newline = true;
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }
}

impl<'src> ReLexer<'src> for PsqlLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match context {
            PsqlReLexContext::TildeName => self.re_lex_tilde_name(),
        };

        if self.current() == re_lexed_kind {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for PsqlLexer<'src> {
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

impl<'src> PsqlLexer<'src> {
    pub fn from_str(source: &'src str, dialect: PsqlDialect) -> Self {
        Self {
            source,
            position: 0,
            after_newline: false,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            diagnostics: vec![],
            dialect,
        }
    }

    fn eat_byte(&mut self, kind: PsqlSyntaxKind) -> PsqlSyntaxKind {
        self.advance(1);
        kind
    }

    fn consume_newline(&mut self) -> bool {
        let start = self.position;
        match self.current_byte() {
            Some(b'\r') if self.peek_byte() == Some(b'\n') => self.advance(2),
            Some(b'\r' | b'\n') => self.advance(1),
            Some(chr) if !chr.is_ascii() => {
                let chr = self.current_char_unchecked();
                if is_linebreak(chr) {
                    self.advance(chr.len_utf8());
                }
            }
            _ => {}
        }
        self.position != start
    }

    fn consume_whitespaces(&mut self) {
        while let Some(chr) = self.current_byte() {
            match lookup_byte(chr) {
                WHS => {
                    if chr == b'\r' || chr == b'\n' {
                        break;
                    } else {
                        self.advance(1);
                    }
                }
                UNI if self.current_char_unchecked().is_whitespace() => {
                    self.advance_char_unchecked();
                }
                _ => break,
            }
        }
    }

    fn resolve_identifier(&mut self) -> PsqlSyntaxKind {
        let start = self.position;
        let mut all_ascii = true;

        while let Some(b) = self.current_byte() {
            if !b.is_ascii() {
                all_ascii = false;
            }
            if !self.advance_id_continue() {
                break;
            }
        }

        // Keywords are always plain ASCII words, so any non-ASCII byte
        // already rules out a keyword match -- no need to look one up.
        if all_ascii {
            let s = &self.source()[start..self.position];
            PsqlSyntaxKind::from_keyword(s.to_lowercase().as_str()).unwrap_or(T![ident])
        } else {
            T![ident]
        }
    }

    /// A `#`-prefixed identifier (e.g. `#tmptable`), valid in the mlang
    /// dialect as a temp-table name -- not real Postgres syntax. Never
    /// resolves to a keyword: `#` can't be part of any real keyword, so
    /// unlike [Self::resolve_identifier] there's no ambiguity to check.
    fn resolve_hash_identifier(&mut self) -> PsqlSyntaxKind {
        self.advance(1); // '#'
        while self.advance_id_continue() {}
        T![ident]
    }

    /// Advances past one identifier-continuation character, ASCII or not
    /// (e.g. Cyrillic, common in real mlang table/column names). Returns
    /// `true` if the current position was such a character. Byte-at-a-time
    /// advancing (`self.advance(1)`) is only correct for ASCII -- a
    /// multi-byte UTF-8 character must advance by its full encoded length,
    /// or later positions land mid-character and panic downstream.
    fn advance_id_continue(&mut self) -> bool {
        match self.current_byte() {
            Some(b) if b.is_ascii() => {
                if is_id_continue(b as char) {
                    self.advance(1);
                    true
                } else {
                    false
                }
            }
            Some(_) => {
                let c = self.current_char_unchecked();
                if is_id_continue(c) {
                    self.advance_char_unchecked();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn consume_quoted_literal(&mut self) -> bool {
        let quote = self.current_byte().unwrap();
        self.advance(1);
        while let Some(b) = self.current_byte() {
            if b == quote {
                self.advance(1);
                // if quote doubled, skip it
                if self.current_byte() == Some(quote) {
                    self.advance(1);
                    continue;
                }
                return true;
            } else if b == b'\\' {
                self.consume_escape_sequence();
            } else {
                self.advance_byte_or_char(b);
            }
        }

        if quote == b'\'' {
            self.push_diagnostic(ParseDiagnostic::new(
                "unterminated string literal",
                self.position..self.position,
            ));
        } else {
            self.push_diagnostic(ParseDiagnostic::new(
                "unterminated quoted identifier",
                self.position..self.position,
            ));
        }

        false
    }

    fn consume_escape_sequence(&mut self) -> bool {
        self.assert_byte(b'\\');
        self.advance(1);
        if let Some(b) = self.current_byte() {
            self.advance_byte_or_char(b);
            true
        } else {
            false
        }
    }

    /// `digit+ ('.' digit*)? (('e'|'E') ('+'|'-')? digit+)?`. Unlike the
    /// previous implementation, this does not greedily swallow `+`/`-`
    /// outside of a well-formed exponent -- those are separate operator
    /// tokens handled by the expression parser (e.g. `1-2` must lex as
    /// `1`, `-`, `2`, not a single `"1-2"` token; negative literals are
    /// `PSQL_UNARY_EXPRESSION`, not part of the lexer).
    fn resolve_number(&mut self) -> PsqlSyntaxKind {
        self.consume_digits();

        if self.current_byte() == Some(b'.') {
            self.advance(1);
            self.consume_digits();
        }

        if matches!(self.current_byte(), Some(b'e' | b'E')) {
            self.consume_exponent();
        }

        PSQL_NUMBER_LITERAL
    }

    fn consume_digits(&mut self) {
        while matches!(self.current_byte(), Some(b'0'..=b'9')) {
            self.advance(1);
        }
    }

    /// Consumes `('e'|'E') ('+'|'-')? digit+`. If there's no digit after the
    /// `e`/sign, backtracks so the `e`/`E` is re-lexed as the start of an
    /// identifier instead of being silently swallowed into an incomplete
    /// exponent (e.g. `1e` lexes as the number `1` followed by the
    /// identifier `e`, not as a single malformed number token).
    fn consume_exponent(&mut self) {
        let start = self.position;
        self.advance(1); // 'e'/'E'
        if matches!(self.current_byte(), Some(b'+' | b'-')) {
            self.advance(1);
        }

        if matches!(self.current_byte(), Some(b'0'..=b'9')) {
            self.consume_digits();
        } else {
            self.position = start;
        }
    }

    #[inline]
    fn resolve_less_than(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'<') => {
                self.next_byte();
                T![<<]
            }
            Some(b'>') => {
                self.next_byte();
                T![<>]
            }
            Some(b'=') => {
                self.next_byte();
                T![<=]
            }
            _ => T![<],
        }
    }

    #[inline]
    fn resolve_more_than(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'>') => {
                T![>>]
            }
            Some(b'=') => {
                self.next_byte();
                T![>=]
            }
            _ => T![>],
        }
    }

    #[inline]
    fn resolve_colon(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b':') => {
                self.next_byte();
                T![::]
            }
            _ => T![:],
        }
    }

    #[inline]
    fn resolve_tilde(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'*') => {
                self.next_byte();
                T![~*]
            }
            // `~~`/`~~*` (`LIKE`/`ILIKE`). Must be lexed as one compound
            // token, not two separate `~`s -- otherwise, in the mlang
            // dialect, the second `~` would land at the start of a new
            // primary expression and get misread as a tilde name.
            Some(b'~') => match self.next_byte() {
                Some(b'*') => {
                    self.next_byte();
                    T![~~*]
                }
                _ => T![~~],
            },
            _ => T![~],
        }
    }

    /// Re-lexes a `~` as the start of a `~name~`/`~$name~` tilde name,
    /// consuming through the matching closing `~`. Only called via
    /// [PsqlReLexContext::TildeName] at grammar points where a name is
    /// expected -- if there's no closing `~` before EOF, this "fails"
    /// (returns the unchanged current kind), leaving the `~` as the plain
    /// operator token it was already lexed as.
    fn re_lex_tilde_name(&mut self) -> PsqlSyntaxKind {
        if self.current_byte() != Some(b'~') {
            return self.current_kind;
        }

        self.advance(1); // opening '~'
        if self.current_byte() == Some(b'$') {
            self.advance(1);
        }

        while let Some(b) = self.current_byte() {
            if b == b'~' {
                self.advance(1);
                return PSQL_TILDE_NAME_LITERAL;
            }
            self.advance_byte_or_char(b);
        }

        self.current_kind
    }

    #[inline]
    fn resolve_bang(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                self.next_byte();
                T![!=]
            }
            Some(b'~') => match self.next_byte() {
                Some(b'*') => {
                    self.next_byte();
                    T![!~*]
                }
                // `!~~`/`!~~*` (`NOT LIKE`/`NOT ILIKE`) -- same reasoning
                // as the `~~`/`~~*` case in `resolve_tilde`.
                Some(b'~') => match self.next_byte() {
                    Some(b'*') => {
                        self.next_byte();
                        T![!~~*]
                    }
                    _ => T![!~~],
                },
                _ => T![!~],
            },
            _ => T![!],
        }
    }

    fn read_minus(&mut self) -> PsqlSyntaxKind {
        self.advance(1); // eats '-'
        if self.current_byte() != Some(b'-') {
            return T![-];
        }

        while let Some(chr) = self.current_byte() {
            if let b'\r' | b'\n' = chr {
                return COMMENT;
            } else if chr.is_ascii() {
                self.advance(1);
            } else {
                let chr = self.current_char_unchecked();
                if is_linebreak(chr) {
                    return COMMENT;
                } else {
                    self.advance(chr.len_utf8());
                }
            }
        }

        COMMENT
    }

    fn read_slash(&mut self) -> PsqlSyntaxKind {
        self.advance(1); // eats '/'
        if self.current_byte() != Some(b'*') {
            return T![/];
        }

        while let Some(chr) = self.current_byte() {
            if let b'*' = chr
                && self.byte_at(1) == Some(b'/')
            {
                self.advance(2);
                return COMMENT;
            } else if chr.is_ascii() {
                self.advance(1);
            } else {
                let chr = self.current_char_unchecked();
                self.advance(chr.len_utf8());
            }
        }

        ERROR_TOKEN
    }

    fn lex_token(&mut self) -> PsqlSyntaxKind {
        let byte = self.current_byte().unwrap_or(0);
        let dispatched = lookup_byte(byte);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            QOT if byte == b'\'' => {
                if self.consume_quoted_literal() {
                    PSQL_STRING_LITERAL
                } else {
                    ERROR_TOKEN
                }
            }
            QOT if byte == b'"' => {
                if self.consume_quoted_literal() {
                    IDENT
                } else {
                    ERROR_TOKEN
                }
            }
            IDT => self.resolve_identifier(),
            // A non-ASCII byte (e.g. the first byte of a Cyrillic letter --
            // common in real mlang table/column names) isn't classified as
            // `IDT` by the byte-level dispatch table (that only covers
            // ASCII); it must be decoded as a full char to tell an
            // identifier character from anything else. Falling through to
            // the generic `_` arm here would be wrong even for non-identifier
            // non-ASCII bytes: that arm only advances one byte, which is
            // correct for ASCII but corrupts later positions (lands mid
            // character) for any multi-byte UTF-8 sequence.
            UNI if is_id_continue(self.current_char_unchecked()) => self.resolve_identifier(),
            UNI => {
                let start = self.position;
                let chr = self.current_char_unchecked();
                self.advance_char_unchecked();
                let err =
                    ParseDiagnostic::new(format!("unexpected token `{chr}`"), start..self.position);
                self.push_diagnostic(err);
                ERROR_TOKEN
            }
            HAS if self.dialect.is_mlang() => self.resolve_hash_identifier(),
            DIG | ZER => self.resolve_number(),
            PNO => self.eat_byte(T!['(']),
            PNC => self.eat_byte(T![')']),
            BTO => self.eat_byte(T!['[']),
            BTC => self.eat_byte(T![']']),
            COM => self.eat_byte(T![,]),
            SEM => self.eat_byte(T![;]),
            PRD => self.eat_byte(T![.]),
            COL => self.resolve_colon(),
            LSS => self.resolve_less_than(),
            MOR => self.resolve_more_than(),
            TLD => self.resolve_tilde(),
            PIP => self.eat_byte(T![|]),
            Dispatch::AMP => self.eat_byte(T![&]),
            CRT => self.eat_byte(T![^]),
            PLS => self.eat_byte(T![+]),
            MIN => self.read_minus(),
            MUL => self.eat_byte(T![*]),
            SLH => self.read_slash(),
            EQL => self.eat_byte(T![=]),
            PRC => self.eat_byte(T![%]),
            EXL => self.resolve_bang(),
            _ => {
                let err = ParseDiagnostic::new(
                    format!("unexpected token `{}`", byte as char),
                    self.position..self.position + 1,
                );
                self.push_diagnostic(err);
                self.advance(1);
                ERROR_TOKEN
            }
        }
    }
}

fn is_linebreak(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
}

fn is_id_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
