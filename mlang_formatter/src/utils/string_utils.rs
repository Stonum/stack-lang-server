use super::concatenation::hole_placeholder;
use crate::prelude::*;

use biome_formatter::{FormatOptions, LineWidth, format_args, write};
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

/// The width-sensitive result of reformatting an embedded query: whether
/// there's a real choice to make between a flat, single-line rendering and
/// a wrapped, multi-line one, or the outcome is already settled either way.
///
/// The point of keeping both candidates (rather than picking one up front)
/// is that how much room is actually available for this text isn't known
/// until print time -- it depends on the ambient indent depth, which other
/// call arguments/expression pieces share the line, and whether the
/// surrounding call has already had to break for unrelated reasons. Only
/// the printer, walking the real document, knows that; see
/// [FormatSqlStringToken]'s use of `best_fitting!` for the [Fits] case,
/// which defers the choice to exactly that point.
///
/// [Fits]: EmbeddedSql::Fits
pub(crate) enum EmbeddedSql {
    /// Fits on one line no matter where it ends up -- even the narrower
    /// `pretty_line_width` budget doesn't force a wrap.
    Flat(String),
    /// Fits on one line *if* there's room; `wrapped` (already reformatted
    /// to fit `pretty_line_width`) is the fallback for when there isn't.
    Fits { flat: String, wrapped: String },
    /// Doesn't fit on one line even at the document's own `line_width` (a
    /// dollar-quoted body or a comment spanning lines verbatim, a select
    /// list too wide for any single line, ...) -- there's no flat candidate
    /// to offer, so this is always multi-line.
    Wrapped(String),
}

impl EmbeddedSql {
    /// Applies a fallible transform (placeholder restoration, see
    /// [try_format_embedded_sql]) to every string this holds.
    fn try_map_strings(self, f: impl Fn(&str) -> Option<String>) -> Option<Self> {
        Some(match self {
            EmbeddedSql::Flat(sql) => EmbeddedSql::Flat(f(&sql)?),
            EmbeddedSql::Fits { flat, wrapped } => EmbeddedSql::Fits {
                flat: f(&flat)?,
                wrapped: f(&wrapped)?,
            },
            EmbeddedSql::Wrapped(sql) => EmbeddedSql::Wrapped(f(&sql)?),
        })
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
pub(crate) fn try_format_embedded_sql(token: &MSyntaxToken, f: &MFormatter) -> Option<EmbeddedSql> {
    let content = token.text_trimmed();
    let raw_content = content.get(1..content.len().saturating_sub(1))?;

    match substitute_format_placeholders(raw_content) {
        Some((substituted, originals)) => format_embedded_sql_variants(&substituted, f)?
            .try_map_strings(|formatted| restore_format_placeholders(formatted, &originals)),
        None => format_embedded_sql_variants(raw_content, f),
    }
}

/// Formats `raw` at the mlang document's own `line_width` to get a
/// candidate flat, single-line rendering -- then, only if that candidate
/// really is single-line, also formats it at the narrower `pretty_line_width`
/// (the width every other nested/wrapped construct in this formatter
/// already reformats against once it's known to need breaking -- call
/// arguments, array elements, object members) to get a fallback multi-line
/// rendering. [FormatSqlStringToken] embeds both candidates in a
/// `best_fitting!` so the printer itself picks whichever actually fits at
/// the real print position.
///
/// `line_width` (not some wider budget) is the right cap for the flat
/// candidate: it's the hard ceiling on every line the printer will ever
/// accept, at any indent depth, so a rendering that doesn't fit within it
/// unindented could never be selected by `best_fitting!` regardless of how
/// much wider a budget generated it -- there'd be no point handing the
/// printer a flat candidate that's mathematically guaranteed to lose.
/// Capping generation at `line_width` instead means `sql_formatter` itself
/// already gives up and wraps once a query passes that point, so this
/// falls straight into the `Wrapped` case below instead of manufacturing a
/// doomed `Fits` candidate first.
///
/// If even that doesn't fit on one line (a dollar-quoted body or comment
/// that must span lines verbatim, a select list too wide for any single
/// line, ...), there's no flat candidate to offer at all -- go straight to
/// reformatting at `pretty_line_width` so the query's *other* clauses still
/// wrap at a sane width instead of stretching out to `line_width`.
fn format_embedded_sql_variants(raw: &str, f: &MFormatter) -> Option<EmbeddedSql> {
    let flat = format_sql_source(raw, f, f.options().line_width())?;

    if flat.lines().count() > 1 {
        let wrapped = format_sql_source(raw, f, f.options().pretty_line_width())?;
        return Some(EmbeddedSql::Wrapped(wrapped));
    }

    let wrapped = format_sql_source(raw, f, f.options().pretty_line_width())?;
    Some(if wrapped.lines().count() > 1 {
        EmbeddedSql::Fits { flat, wrapped }
    } else {
        EmbeddedSql::Flat(flat)
    })
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
/// `sql_formatter` at the given `line_width`, matching the surrounding
/// mlang code's indent style. Returns `None` on any parse/format failure.
/// Shared by the single-literal path ([format_embedded_sql_variants], which
/// calls this at two different widths to build its flat/wrapped
/// candidates) and the concatenation-chain path (`utils/concatenation.rs`),
/// which parses a placeholder-substituted join of several string-literal
/// pieces the same way.
pub(crate) fn format_sql_source(
    raw: &str,
    f: &MFormatter,
    line_width: LineWidth,
) -> Option<String> {
    let syntax = sql_syntax::SqlFileSource::query()
        .with_dialect(sql_syntax::SqlDialect::Postgres)
        .with_mlang_extension(true);
    let parsed = sql_parser::parse(raw, syntax);
    if parsed.has_errors() {
        return None;
    }

    // `sql_formatter` echoes multi-line comments and dollar-quoted string
    // bodies back byte-for-byte, whatever leading whitespace they already
    // have (see `format_create_function_dollar_quoted_body_is_preserved_verbatim`
    // in `sql_formatter`). But this function's own output gets re-embedded
    // one indent level deeper by `format_reformatted_multi_line_query`,
    // which bakes that extra indent in as literal characters -- including
    // inside these untouched spans. Reformatting *that* output again feeds
    // the now-shifted spans straight back in unchanged, and another layer
    // of ambient indent stacks on top: unbounded growth across repeated
    // format passes. Stripping each span's own common leading whitespace
    // before parsing removes exactly the layer the *previous* pass added,
    // without touching indentation that's actually part of the content.
    let dedented = dedent_verbatim_spans(raw, &parsed.syntax());
    let parsed = match dedented {
        Cow::Borrowed(_) => parsed,
        Cow::Owned(ref text) => {
            let reparsed = sql_parser::parse(text, syntax);
            if reparsed.has_errors() {
                parsed
            } else {
                reparsed
            }
        }
    };

    // Match the embedded query's own indentation to the surrounding mlang
    // code's, since the resulting lines get spliced in as raw text (see
    // `format_reformatted_multi_line_query`) -- a mismatch would otherwise
    // mix, say, mlang's spaces with sql_formatter's default tabs. The line
    // width is the caller's call: see [format_embedded_sql_variants].
    let options = sql_formatter::SqlFormatOptions::new(syntax)
        .with_indent_style(f.options().indent_style())
        .with_indent_width(f.options().indent_width())
        .with_line_width(line_width)
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

/// Finds every multi-line comment and multi-line token (a dollar-quoted or
/// otherwise multi-line string literal -- see the note in
/// `sql_formatter/src/rules/tokens.rs` on why those are the only multi-line
/// tokens this grammar produces) in `tree`, and dedents each one's interior
/// lines (everything but the first) to their own common minimum leading
/// whitespace. `Cow::Borrowed(raw)` if nothing needed stripping.
fn dedent_verbatim_spans<'a>(raw: &'a str, tree: &sql_syntax::SqlSyntaxNode) -> Cow<'a, str> {
    let mut spans: Vec<(usize, usize, String)> = Vec::new();

    for token in tree.descendants_tokens(biome_rowan::Direction::Next) {
        let trimmed = token.text_trimmed();
        if trimmed.contains('\n')
            && let Some(dedented) = dedent_span(trimmed)
        {
            let start = u32::from(token.text_trimmed_range().start()) as usize;
            spans.push((start, start + trimmed.len(), dedented));
        }

        for piece in token
            .leading_trivia()
            .pieces()
            .chain(token.trailing_trivia().pieces())
        {
            let Some(comment) = piece.as_comments() else {
                continue;
            };
            let text = comment.text();
            if text.contains('\n')
                && let Some(dedented) = dedent_span(text)
            {
                let start = u32::from(comment.text_range().start()) as usize;
                spans.push((start, start + text.len(), dedented));
            }
        }
    }

    if spans.is_empty() {
        return Cow::Borrowed(raw);
    }

    spans.sort_by_key(|(start, ..)| *start);

    let mut result = String::with_capacity(raw.len());
    let mut cursor = 0;
    for (start, end, dedented) in spans {
        result.push_str(&raw[cursor..start]);
        result.push_str(&dedented);
        cursor = end;
    }
    result.push_str(&raw[cursor..]);

    Cow::Owned(result)
}

/// Dedents `span`'s interior lines (everything after the first) to their
/// own common minimum leading whitespace. `None` if there's nothing to
/// strip -- a single line, or already at a zero common minimum -- so the
/// caller can tell "no span-worthy change" apart from "dedented to
/// nothing", without allocating in the common case.
pub(crate) fn dedent_span(span: &str) -> Option<String> {
    let mut lines = span.split('\n');
    let first = lines.next()?;

    let common_indent = lines
        .clone()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start_matches([' ', '\t']).len())
        .min()?;

    if common_indent == 0 {
        return None;
    }

    let mut result = String::with_capacity(span.len());
    result.push_str(first);
    for line in lines {
        result.push('\n');
        if line.trim().is_empty() {
            continue;
        }
        result.push_str(&line[common_indent..]);
    }
    Some(result)
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

    /// Splices in a `best_fitting!` choice between `flat_sql` (single-line)
    /// and `wrapped_sql` (already reformatted multi-line, see
    /// [format_reformatted_multi_line_query][Self::format_reformatted_multi_line_query])
    /// -- the printer measures `flat_sql` against the real column position
    /// and whatever else shares the line at print time, falling back to
    /// `wrapped_sql` only if it genuinely doesn't fit. Both variants must
    /// commit to the same delimiter up front, since which one gets printed
    /// isn't decided until later.
    fn format_best_fitting_query(
        &self,
        flat_sql: String,
        wrapped_sql: String,
        preferred_quote: char,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let start = self.token.text_trimmed_range().start();
        let quote = if effective_quote(preferred_quote, &flat_sql) == '`'
            || effective_quote(preferred_quote, &wrapped_sql) == '`'
        {
            '`'
        } else {
            preferred_quote
        };
        let quote_text = quote_as_static_str(quote);

        let flat_escaped = escape_for_string_literal(&flat_sql, quote).into_owned();

        write!(
            f,
            [format_replaced(
                self.token,
                &best_fitting![
                    format_args![
                        text(quote_text),
                        dynamic_text(&flat_escaped, start),
                        text(quote_text),
                    ],
                    format_args![
                        text(quote_text),
                        block_indent(&format_with(move |f| {
                            let mut lines = wrapped_sql.lines();
                            if let Some(first) = lines.next() {
                                write!(
                                    f,
                                    [dynamic_text(
                                        &escape_for_string_literal(first, quote),
                                        start
                                    )]
                                )?;
                                for line in lines {
                                    write!(
                                        f,
                                        [
                                            hard_line_break(),
                                            dynamic_text(
                                                &escape_for_string_literal(line, quote),
                                                start
                                            )
                                        ]
                                    )?;
                                }
                            }
                            Ok(())
                        })),
                        text(quote_text),
                    ],
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

/// Reformatting can introduce `"`-quoted identifiers (e.g. an mlang
/// `[bracket]` identifier canonicalized by `sql_formatter` to Postgres's
/// own `"..."` spelling) that would otherwise have to be escaped to fit
/// inside a `"`-delimited mlang string. Since mlang's `` ` `` and `"``
/// delimiters are interchangeable (see [quote_as_static_str]'s doc
/// comment), prefer switching to `` ` `` over escaping -- but only when the
/// formatted SQL actually contains the conflicting quote; a query that
/// stays clean in double quotes keeps its original delimiter untouched.
fn effective_quote(preferred_quote: char, formatted_sql: &str) -> char {
    if preferred_quote == '"' && formatted_sql.contains('"') {
        '`'
    } else {
        preferred_quote
    }
}

impl Format<MFormatContext> for FormatSqlStringToken<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        // Preserve whichever delimiter the query was actually written
        // with -- mlang's lexer treats `` ` `` and `"` identically (see
        // `quote_as_static_str`'s doc comment), so there's no reason to
        // force one over the other.
        let preferred_quote = self.token().text_trimmed().chars().next().unwrap_or('`');

        if let Some(embedded) = try_format_embedded_sql(self.token(), f) {
            return match embedded {
                EmbeddedSql::Flat(sql) => {
                    let quote = effective_quote(preferred_quote, &sql);
                    let escaped = escape_for_string_literal(&sql, quote);
                    let content: Cow<str> = Cow::Owned(std::format!("{quote}{escaped}{quote}"));
                    self.format_single_line_query(content, f)
                }
                EmbeddedSql::Wrapped(sql) => {
                    let quote = effective_quote(preferred_quote, &sql);
                    self.format_reformatted_multi_line_query(sql, quote, f)
                }
                EmbeddedSql::Fits { flat, wrapped } => {
                    self.format_best_fitting_query(flat, wrapped, preferred_quote, f)
                }
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
