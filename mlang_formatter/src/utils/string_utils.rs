use super::concatenation::hole_placeholder;
use crate::prelude::*;

use biome_formatter::{FormatOptions, format_args, write};
use mlang_syntax::MSyntaxKind::{M_LONG_STRING_LITERAL, M_STRING_LITERAL};
use mlang_syntax::MSyntaxToken;
use mlang_syntax::concatenation::substitute_format_placeholders;
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

    pub fn text(&self) -> &str {
        &self.text
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
/// normalises `\r\n`/`\r` line endings) string content as embedded SQL and
/// reformat it with `sql_formatter`.
///
/// Returns `None` if the content doesn't parse cleanly, in which case the
/// caller must leave the string exactly as written. This is a deliberate
/// safety net: a formatter must never risk corrupting a real query it
/// doesn't fully understand (unsupported syntax, string concatenation
/// building up the query in pieces, etc.) just to make it "prettier".
pub(crate) fn try_format_embedded_sql(token: &MSyntaxToken, f: &MFormatter) -> Option<String> {
    let content = token.text_trimmed();
    let raw_content = content.get(1..content.len().saturating_sub(1))?;

    match substitute_format_placeholders(raw_content) {
        Some((substituted, originals)) => {
            let formatted = format_sql_source(&substituted, f)?;
            restore_format_placeholders(&formatted, &originals)
        }
        None => format_sql_source(raw_content, f),
    }
}

/// Reverses [substitute_format_placeholders]: finds each placeholder in
/// `formatted`, in order, and splices the corresponding original `{...}`
/// text back in. `None` if a placeholder can't be found.
fn restore_format_placeholders(formatted: &str, originals: &[String]) -> Option<String> {
    let mut result = String::with_capacity(formatted.len());
    let mut cursor = 0;
    for (index, original) in originals.iter().enumerate() {
        let placeholder = hole_placeholder(index);
        let found_at = cursor + formatted.get(cursor..)?.find(&placeholder)?;
        result.push_str(&formatted[cursor..found_at]);
        result.push_str(original);
        cursor = found_at + placeholder.len();
    }
    result.push_str(&formatted[cursor..]);

    Some(result)
}

/// Parses `raw` as embedded SQL (Postgres dialect + the `mlang` extension,
/// so `~table~`/`#temp`/`:param` are understood) and reformats it with
/// `sql_formatter`, matching the surrounding mlang code's indent
/// style/width. Returns `None` on any parse/format failure. Shared by the
/// single-literal path ([try_format_embedded_sql]) and the
/// concatenation-chain path (`utils/concatenation.rs`), which parses a
/// placeholder-substituted join of several string-literal pieces the same
/// way.
pub(crate) fn format_sql_source(raw: &str, f: &MFormatter) -> Option<String> {
    let syntax = sql_syntax::SqlFileSource::query()
        .with_dialect(sql_syntax::SqlDialect::Postgres)
        .with_mlang_extension(true);
    let parsed = sql_parser::parse(raw, syntax);
    if parsed.has_errors() {
        return None;
    }

    // Match the embedded query's own indentation to the surrounding mlang
    // code's style/width, since the resulting lines get spliced in as raw
    // text (see `format_reformatted_multi_line_query`) -- a mismatch would
    // otherwise mix, say, mlang's spaces with sql_formatter's default tabs.
    let options = sql_formatter::SqlFormatOptions::new(syntax)
        .with_indent_style(f.options().indent_style())
        .with_indent_width(f.options().indent_width())
        // Legacy mlang queries use SQL-Server-style `[bracket]` identifiers
        // even where they're otherwise ordinary, valid Postgres -- always
        // normalize those to Postgres's own `"..."` spelling when
        // reformatting embedded SQL (matches `FormatSqlStringToken`'s own
        // delimiter-switching logic just below, which already assumes `"`
        // may appear in the output).
        .with_bracket_identifier_style(sql_formatter::BracketIdentifierStyle::ConvertToQuotes);
    let formatted = sql_formatter::format_node(options, &parsed.syntax()).ok()?;
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

    /// Splices freshly `sql_formatter`-formatted SQL text in as this
    /// token's new content, using `preferred_quote` -- the token's own
    /// original delimiter (`` ` `` or `"`; mlang lexes both identically,
    /// including support for literal embedded newlines, so there's no
    /// reason to force one over the other) -- as the surrounding quote.
    /// Unlike [Self::format_multi_line_query], every line (including the
    /// first) is indented uniformly one level via a single `block_indent`
    /// -- the per-line block-indent-vs-dedent branching there exists to
    /// approximate the *original* author's own indentation style when
    /// reproducing raw content close to verbatim, which doesn't apply here
    /// since `sql_formatter`'s own output is already internally consistent.
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
/// `text()`. Shared with the concatenation-chain path
/// (`utils/concatenation.rs`), which wraps each reformatted literal segment
/// back into a valid mlang string literal the same way.
pub(crate) fn quote_as_static_str(quote: char) -> &'static str {
    if quote == '`' { "`" } else { "\"" }
}

/// Escapes a bare (unescaped) `quote` delimiter character in `text` so it
/// can be embedded inside a `quote`-delimited mlang string literal without
/// prematurely terminating it (e.g. a double-quoted mlang string around SQL
/// containing `"quoted identifiers"`). `text` is typically a mix of fresh
/// output (which may contain such a bare `quote`) and pass-through raw
/// mlang source content, where `\` always already starts a valid two-plus-
/// character escape sequence (mlang's lexer accepts `\` followed by *any*
/// character as one escape unit, see `consume_escape_sequence`) -- such a
/// sequence is left untouched rather than escaped again, or every reformat
/// pass would keep adding another backslash. Shared with the
/// concatenation-chain path (`utils/concatenation.rs`).
pub(crate) fn escape_for_string_literal(text: &str, quote: char) -> Cow<'_, str> {
    if !text.contains(['\\', quote]) {
        return Cow::Borrowed(text);
    }

    let mut escaped = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            escaped.push(c);
            if let Some(next) = chars.next() {
                escaped.push(next);
            }
        } else {
            if c == quote {
                escaped.push('\\');
            }
            escaped.push(c);
        }
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
            // Reformatting can introduce `"`-quoted identifiers (e.g. an
            // mlang `[bracket]` identifier canonicalized by
            // `sql_formatter` to Postgres's own `"..."` spelling) that
            // would otherwise have to be escaped to fit inside a
            // `"`-delimited mlang string. Since mlang's `` ` `` and `"`
            // delimiters are interchangeable (see `quote_as_static_str`'s
            // doc comment), prefer switching to `` ` `` over escaping --
            // but only when the formatted SQL actually contains the
            // conflicting quote; a query that stays clean in double quotes
            // keeps its original delimiter untouched.
            let effective_quote = if preferred_quote == '"' && formatted_sql.contains('"') {
                '`'
            } else {
                preferred_quote
            };

            return if formatted_sql.lines().count() > 1 {
                self.format_reformatted_multi_line_query(formatted_sql, effective_quote, f)
            } else {
                let escaped = escape_for_string_literal(&formatted_sql, effective_quote);
                let content: Cow<str> =
                    Cow::Owned(std::format!("{effective_quote}{escaped}{effective_quote}"));
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

#[cfg(test)]
mod format_placeholder_tests {
    use super::*;

    #[test]
    fn substitutes_and_restores_round_trip() {
        let raw = "select {0} from t where a = {1}";
        let (substituted, originals) = substitute_format_placeholders(raw).unwrap();

        assert_eq!(originals, vec!["{0}", "{1}"]);
        assert!(!substituted.contains('{'));

        let restored = restore_format_placeholders(&substituted, &originals).unwrap();
        assert_eq!(restored, raw);
    }
}
