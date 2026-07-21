use crate::prelude::*;

use biome_formatter::{FormatOptions, format_args, write};
use mlang_syntax::MSyntaxKind::{M_LONG_STRING_LITERAL, M_STRING_LITERAL};
use mlang_syntax::MSyntaxToken;
use std::borrow::Cow;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum StringLiteralParentKind {
    /// Variant to track tokens that are inside an expression
    Expression,
    /// Variant to track tokens that are inside a member
    Member,
}

/// Data structure of convenience to format string literals
pub(crate) struct FormatLiteralStringToken<'token> {
    token: &'token MSyntaxToken,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token MSyntaxToken, _parent_kind: StringLiteralParentKind) -> Self {
        debug_assert!(
            matches!(token.kind(), M_STRING_LITERAL | M_LONG_STRING_LITERAL),
            "Found kind {:?}",
            token.kind()
        );

        Self { token }
    }

    fn token(&self) -> &'token MSyntaxToken {
        self.token
    }

    pub fn clean_text(&self) -> CleanedStringLiteralText<'_> {
        let token = self.token();

        let content = token.text_trimmed();
        let preferred_quote = content.chars().next().unwrap_or('"');

        let mut string_cleaner = LiteralStringNormaliser::new(token, preferred_quote);

        let content = string_cleaner.normalise_text();
        let normalized_text_width = content.len();

        CleanedStringLiteralText {
            text: content,
            width: normalized_text_width,
            token,
        }
    }
}

pub(crate) struct CleanedStringLiteralText<'a> {
    token: &'a MSyntaxToken,
    text: Cow<'a, str>,
    width: usize,
}

impl CleanedStringLiteralText<'_> {
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Format<MFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

impl Format<MFormatContext> for FormatLiteralStringToken<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        let cleaned = self.clean_text();

        cleaned.fmt(f)
    }
}

/// Struct of convenience used to manipulate the string. It saves some state in order to apply
/// the normalise process.
struct LiteralStringNormaliser<'token> {
    /// The current token
    token: &'token MSyntaxToken,
    preferred_quote: char,
}

impl<'token> LiteralStringNormaliser<'token> {
    pub fn new(token: &'token MSyntaxToken, preferred_quote: char) -> Self {
        Self {
            token,
            preferred_quote,
        }
    }

    fn normalise_text(&mut self) -> Cow<'token, str> {
        self.normalise_string_literal()
    }

    fn normalise_string_literal(&mut self) -> Cow<'token, str> {
        let polished_raw_content = self.normalize_string();
        let preferred_quote = self.preferred_quote;

        match polished_raw_content {
            Cow::Borrowed(raw_content) => self.swap_quotes(raw_content, preferred_quote),
            Cow::Owned(mut s) => {
                // content is owned, meaning we allocated a new string,
                // so we force replacing quotes, regardless
                s.insert(0, preferred_quote);
                s.push(preferred_quote);
                Cow::Owned(s)
            }
        }
    }

    fn normalize_string(&self) -> Cow<'token, str> {
        let raw_content = self.raw_content();
        let mut reduced_string = String::new();
        let mut copy_start = 0;
        let mut bytes = raw_content.bytes().enumerate();

        while let Some((byte_index, byte)) = bytes.next() {
            match byte {
                // If the next character is escaped
                b'\\' => {
                    if let Some((escaped_index, escaped)) = bytes.next()
                        && escaped == b'\r'
                    {
                        // If we encounter the sequence "\r\n", then skip '\r'
                        if let Some((next_byte_index, b'\n')) = bytes.next() {
                            reduced_string.push_str(&raw_content[copy_start..escaped_index]);
                            copy_start = next_byte_index;
                        }
                    }
                }
                // If we encounter the sequence "\r\n", then skip '\r'
                b'\r' => {
                    if let Some((next_byte_index, b'\n')) = bytes.next() {
                        reduced_string.push_str(&raw_content[copy_start..byte_index]);
                        copy_start = next_byte_index;
                    }
                }
                _ => {}
            }
        }
        if copy_start == 0 && reduced_string.is_empty() {
            Cow::Borrowed(raw_content)
        } else {
            // Copy the remaining characters
            reduced_string.push_str(&raw_content[copy_start..]);
            Cow::Owned(reduced_string)
        }
    }

    /// Returns the string without its quotes.
    fn raw_content(&self) -> &'token str {
        let content = self.token.text_trimmed();
        &content[1..content.len() - 1]
    }

    fn swap_quotes(&self, content_to_use: &'token str, preferred_quote: char) -> Cow<'token, str> {
        let original = self.token.text_trimmed();

        if original.starts_with(preferred_quote) {
            Cow::Borrowed(original)
        } else {
            Cow::Owned(std::format!(
                "{preferred_quote}{content_to_use}{preferred_quote}",
            ))
        }
    }
}

/// Tries to parse `token`'s raw (still-escaped, as written in source --
/// `LiteralStringNormaliser` doesn't decode escapes either, it only
/// normalises `\r\n`/`\r` line endings) string content as embedded SQL
/// (mlang dialect, so `~table~`/`#temp`/`:param` are understood) and
/// reformat it with `psql_formatter`.
///
/// Returns `None` if the content doesn't parse cleanly, in which case the
/// caller must leave the string exactly as written. This is a deliberate
/// safety net: a formatter must never risk corrupting a real query it
/// doesn't fully understand (unsupported syntax, string concatenation
/// building up the query in pieces, etc.) just to make it "prettier".
pub(crate) fn try_format_embedded_sql(token: &MSyntaxToken, f: &MFormatter) -> Option<String> {
    let content = token.text_trimmed();
    let raw_content = content.get(1..content.len().saturating_sub(1))?;

    let syntax = psql_syntax::PsqlFileSource::query().with_dialect(psql_syntax::PsqlDialect::Mlang);
    let parsed = psql_parser::parse(raw_content, syntax);
    if parsed.has_errors() {
        return None;
    }

    // Match the embedded query's own indentation to the surrounding mlang
    // code's style/width, since the resulting lines get spliced in as raw
    // text (see `format_reformatted_multi_line_query`) -- a mismatch would
    // otherwise mix, say, mlang's spaces with psql_formatter's default tabs.
    let options = psql_formatter::PsqlFormatOptions::new(syntax)
        .with_indent_style(f.options().indent_style())
        .with_indent_width(f.options().indent_width());
    let formatted = psql_formatter::format_node(options, &parsed.syntax()).ok()?;
    let printed = formatted.print().ok()?;

    Some(printed.as_code().trim_end().to_string())
}

pub(crate) struct FormatSqlStringToken<'token> {
    token: &'token MSyntaxToken,
}

impl<'token> FormatSqlStringToken<'token> {
    pub fn new(token: &'token MSyntaxToken) -> Self {
        Self { token }
    }

    fn token(&self) -> &'token MSyntaxToken {
        self.token
    }

    fn format_single_line_query(
        &self,
        content: Cow<'token, str>,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [format_replaced(
                self.token,
                &syntax_token_cow_slice(
                    content,
                    self.token,
                    self.token.text_trimmed_range().start()
                ),
            ),]
        )
    }

    fn format_multi_line_query(
        &self,
        content: Cow<'token, str>,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let start = self.token.text_trimmed_range().start();
        let quoteless = &content[1..content.len() - 1];

        // if query starts with newline - write it as is
        if quoteless.trim_start_matches([' ', '\t']).starts_with("\n") {
            return write!(
                f,
                [format_replaced(
                    self.token,
                    &syntax_token_cow_slice(content, self.token, start,),
                )]
            );
        }

        let quoteless = quoteless.trim_start_matches('\n').trim_end();

        write!(
            f,
            [format_replaced(
                self.token,
                &format_args![
                    text("`"),
                    &format_with(move |f| {
                        for (index, line) in quoteless.lines().enumerate() {
                            match index {
                                // write on new line with indent
                                0 if !line.starts_with(' ') => write!(
                                    f,
                                    [block_indent(&format_args![
                                        dynamic_text(line, start),
                                        hard_line_break(),
                                    ])]
                                )?,

                                // write on new line with dedent of parent
                                0 => write!(
                                    f,
                                    [dedent(&format_args![
                                        hard_line_break(),
                                        dynamic_text(line, start),
                                        hard_line_break(),
                                    ]),]
                                )?,

                                _ => write!(f, [dynamic_text(line, start), hard_line_break(),])?,
                            }
                        }
                        Ok(())
                    }),
                    text("`")
                ]
            )]
        )
    }

    /// Splices freshly `psql_formatter`-formatted SQL text in as this
    /// token's new content, using `preferred_quote` -- the token's own
    /// original delimiter (`` ` `` or `"`; mlang lexes both identically,
    /// including support for literal embedded newlines, so there's no
    /// reason to force one over the other) -- as the surrounding quote.
    /// Unlike [Self::format_multi_line_query], every line (including the
    /// first) is indented uniformly one level via a single `block_indent`
    /// -- the per-line block-indent-vs-dedent branching there exists to
    /// approximate the *original* author's own indentation style when
    /// reproducing raw content close to verbatim, which doesn't apply here
    /// since `psql_formatter`'s own output is already internally consistent.
    fn format_reformatted_multi_line_query(
        &self,
        formatted_sql: String,
        preferred_quote: char,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let start = self.token.text_trimmed_range().start();
        let quote_text = quote_as_static_str(preferred_quote);

        write!(
            f,
            [format_replaced(
                self.token,
                &format_args![
                    text(quote_text),
                    block_indent(&format_with(move |f| {
                        let mut lines = formatted_sql.lines();
                        if let Some(first) = lines.next() {
                            write!(
                                f,
                                [dynamic_text(
                                    &escape_for_string_literal(first, preferred_quote),
                                    start
                                )]
                            )?;
                            for line in lines {
                                write!(
                                    f,
                                    [
                                        hard_line_break(),
                                        dynamic_text(
                                            &escape_for_string_literal(line, preferred_quote),
                                            start
                                        )
                                    ]
                                )?;
                            }
                        }
                        Ok(())
                    })),
                    text(quote_text),
                ]
            )]
        )
    }
}

/// The only two delimiters mlang's lexer accepts for a string literal (see
/// `consume_str_literal` in `mlang_parser`'s lexer -- `'` is reserved for
/// "long identifiers", not strings), as a `'static` string for use with
/// `text()`.
fn quote_as_static_str(quote: char) -> &'static str {
    if quote == '`' { "`" } else { "\"" }
}

/// Escapes `\` and the delimiter `quote` character (mlang's string escape
/// rules recognise `\\`/`` \` ``/`\"`, per `consume_escape_sequence`) so
/// `text` can be embedded inside a `quote`-delimited mlang string literal
/// without prematurely terminating it (e.g. a double-quoted mlang string
/// around SQL containing `"quoted identifiers"`) or corrupting an
/// accidental escape sequence.
fn escape_for_string_literal(text: &str, quote: char) -> Cow<'_, str> {
    if !text.contains(['\\', quote]) {
        return Cow::Borrowed(text);
    }

    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if c == '\\' || c == quote {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    Cow::Owned(escaped)
}

impl Format<MFormatContext> for FormatSqlStringToken<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        // Preserve whichever delimiter the query was actually written
        // with -- mlang's lexer treats `` ` `` and `"` identically (see
        // `quote_as_static_str`'s doc comment), so there's no reason to
        // force one over the other.
        let preferred_quote = self.token().text_trimmed().chars().next().unwrap_or('`');

        if let Some(formatted_sql) = try_format_embedded_sql(self.token(), f) {
            return if formatted_sql.lines().count() > 1 {
                self.format_reformatted_multi_line_query(formatted_sql, preferred_quote, f)
            } else {
                let escaped = escape_for_string_literal(&formatted_sql, preferred_quote);
                let content: Cow<str> =
                    Cow::Owned(std::format!("{preferred_quote}{escaped}{preferred_quote}"));
                self.format_single_line_query(content, f)
            };
        }

        let token = self.token();
        let mut string_cleaner = LiteralStringNormaliser::new(token, preferred_quote);

        let content = string_cleaner.normalise_text();
        if content.lines().count() > 1 {
            self.format_multi_line_query(content, f)
        } else {
            self.format_single_line_query(content, f)
        }
    }
}
