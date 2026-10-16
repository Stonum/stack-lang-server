//! Formats the SQL inside a PL/pgSQL `EXECUTE` string: a string literal or a
//! `'...' || expr || '...'` chain is joined into one query (holes become
//! placeholders), formatted, and split back into pieces -- the same approach
//! as SQL strings embedded in mlang.

use crate::LineWidth;
use crate::prelude::*;
use biome_formatter::{FormatOptions, write};
use biome_rowan::{AstNode, TextSize};
use sql_syntax::{
    AnySqlExpression, AnySqlLiteralExpression, SqlLanguage, SqlSyntaxKind, SqlSyntaxNode,
    SqlSyntaxToken, T,
};

const HOLE_PREFIX: &str = "__sql_hole_";

/// A hole is first tried as an identifier (a table name, a value); one
/// that holds a fragment (`... ' || condition || ' ...`) then becomes a
/// comment, which the parser skips.
fn hole_placeholder(index: usize, as_comment: bool) -> String {
    if as_comment {
        std::format!("/*{HOLE_PREFIX}{index}__*/")
    } else {
        std::format!("{HOLE_PREFIX}{index}__")
    }
}

/// Whether `comment` is a hole placeholder of [EmbeddedQuery].
pub(crate) fn is_hole_comment(comment: &str) -> bool {
    comment.starts_with("/*") && comment.contains(HOLE_PREFIX)
}

/// Whether `expression` contains a placeholder of [EmbeddedQuery].
pub(crate) fn contains_hole_placeholder(expression: &SqlSyntaxNode) -> bool {
    expression
        .descendants_tokens(biome_rowan::Direction::Next)
        .any(|token| {
            token.kind() == SqlSyntaxKind::IDENT && token.text_trimmed().contains(HOLE_PREFIX)
        })
}

enum Part {
    Literal(Vec<SqlSyntaxToken>),
    Hole(AnySqlExpression),
}

enum Piece {
    Literal(String),
    Hole(usize),
}

pub(crate) struct EmbeddedQuery {
    parts: Vec<Part>,
    operators: Vec<SqlSyntaxToken>,
    pieces: Vec<Piece>,
}

impl EmbeddedQuery {
    /// `None` unless `command` is a plain string literal or a `||` chain of
    /// them and expressions whose joined text parses and formats cleanly.
    pub(crate) fn try_new(command: &AnySqlExpression, f: &SqlFormatter) -> Option<Self> {
        let mut parts = Vec::new();
        let mut operators = Vec::new();
        let mut bypassed = Vec::new();
        collect_parts(command, &mut parts, &mut operators, &mut bypassed)?;
        if !parts.iter().any(|part| matches!(part, Part::Literal(_))) {
            return None;
        }

        let comments = f.comments();
        let mut has_comments = false;
        for node in &bypassed {
            comments.mark_suppression_checked(node);
            has_comments |= comments.has_comments(node);
        }
        if has_comments {
            return None;
        }

        let mut comment_holes = vec![false; parts.len()];
        let parsed = loop {
            let (joined, offsets) = join_parts(&parts, &comment_holes)?;
            let parsed = sql_parser::parse(&joined, f.options().source_type());
            let Some(error) = first_error_offset(&parsed) else {
                break parsed;
            };
            // Retry with the hole closest before the error as a comment.
            let index = offsets
                .iter()
                .filter(|(index, offset)| !comment_holes[*index] && *offset <= error)
                .map(|(index, _)| *index)
                .next_back()?;
            comment_holes[index] = true;
        };
        if has_multi_line_token(&parsed.syntax()) {
            return None;
        }

        let formatted = format_query(&parsed.syntax(), command.syntax(), f)?;
        // The formatter must not move a hole to another place between tokens.
        let reparsed = sql_parser::parse(&formatted, f.options().source_type());
        if reparsed.has_errors()
            || hole_positions(&reparsed.syntax()) != hole_positions(&parsed.syntax())
        {
            return None;
        }
        let pieces = split_pieces(&formatted, &parts, &comment_holes)?;

        Some(Self {
            parts,
            operators,
            pieces,
        })
    }
}

fn collect_parts(
    expression: &AnySqlExpression,
    parts: &mut Vec<Part>,
    operators: &mut Vec<SqlSyntaxToken>,
    bypassed: &mut Vec<SqlSyntaxNode>,
) -> Option<()> {
    if let AnySqlExpression::SqlBinaryExpression(binary) = expression {
        let operator = binary.operator_token().ok()?;
        if operator.kind() == T![||] {
            bypassed.push(binary.syntax().clone());
            collect_parts(&binary.left().ok()?, parts, operators, bypassed)?;
            operators.push(operator);
            return collect_parts(&binary.right().ok()?, parts, operators, bypassed);
        }
    }

    if let AnySqlExpression::AnySqlLiteralExpression(
        AnySqlLiteralExpression::SqlStringLiteralExpression(literal),
    ) = expression
    {
        let token = literal.value_token().ok()?;
        unescape(token.text_trimmed())?;
        bypassed.push(literal.syntax().clone());
        // Adjacent literals (`'a' || 'b'`) merge into one piece.
        if let Some(Part::Literal(tokens)) = parts.last_mut() {
            tokens.push(token);
        } else {
            parts.push(Part::Literal(vec![token]));
        }
        return Some(());
    }

    // Two adjacent holes would merge into one placeholder.
    if matches!(parts.last(), Some(Part::Hole(_))) {
        return None;
    }
    parts.push(Part::Hole(expression.clone()));
    Some(())
}

/// The content of a plain `'...'` literal with `''` unescaped; `None` for
/// any other spelling (`E'...'`, dollar quotes, bit strings, continuations).
fn unescape(text: &str) -> Option<String> {
    let inner = text.strip_prefix('\'')?.strip_suffix('\'')?;
    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\'' && chars.next() != Some('\'') {
            return None;
        }
        result.push(c);
    }
    Some(result)
}

/// The joined query and the offset of each hole placeholder in it.
fn join_parts(parts: &[Part], comment_holes: &[bool]) -> Option<(String, Vec<(usize, usize)>)> {
    let mut joined = String::new();
    let mut offsets = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        match part {
            Part::Literal(tokens) => {
                for token in tokens {
                    let content = unescape(token.text_trimmed())?;
                    if content.contains(HOLE_PREFIX) {
                        return None;
                    }
                    joined.push_str(&content);
                }
            }
            Part::Hole(_) => {
                offsets.push((index, joined.len()));
                joined.push_str(&hole_placeholder(index, comment_holes[index]));
            }
        }
    }
    Some((joined, offsets))
}

fn first_error_offset(parsed: &sql_parser::Parse<sql_syntax::SqlRoot>) -> Option<usize> {
    use biome_diagnostics::Diagnostic;

    parsed
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.is_error())
        .map(|diagnostic| {
            diagnostic
                .location()
                .span
                .map_or(usize::MAX, |span| usize::from(span.start()))
        })
        .min()
}

/// Formats the query at the width left after the indentation of the pieces.
fn format_query(root: &SqlSyntaxNode, command: &SqlSyntaxNode, f: &SqlFormatter) -> Option<String> {
    let options = f.options();
    let depth = command
        .ancestors()
        .filter(|node| node.kind() == SqlSyntaxKind::PSQL_PL_STATEMENT_LIST)
        .count()
        + 1;
    let indent = depth * usize::from(options.indent_width().value());
    let width = (usize::from(options.line_width().get()))
        .saturating_sub(indent)
        .max(40);
    let options = options
        .clone()
        .with_line_width(LineWidth::try_from(width as u16).ok()?);

    let formatted = crate::format_node(options, root).ok()?;
    let printed = formatted.print().ok()?;
    Some(printed.as_code().trim_end().to_string())
}

/// A multi-line comment or string is printed verbatim, so the indentation
/// added to the pieces would grow on every pass.
fn has_multi_line_token(root: &biome_rowan::SyntaxNode<SqlLanguage>) -> bool {
    root.descendants_tokens(biome_rowan::Direction::Next)
        .any(|token| token.text().trim().contains('\n'))
}

/// For each hole in order: how many tokens precede it, not counting
/// parentheses (the formatter may drop redundant ones).
fn hole_positions(root: &SqlSyntaxNode) -> Vec<usize> {
    let holes_in = |text: &str| text.matches(HOLE_PREFIX).count();
    let mut positions = Vec::new();
    let mut count = 0;
    for token in root.descendants_tokens(biome_rowan::Direction::Next) {
        for piece in token.leading_trivia().pieces() {
            positions.extend(std::iter::repeat_n(count, holes_in(piece.text())));
        }
        positions.extend(std::iter::repeat_n(count, holes_in(token.text_trimmed())));
        if !matches!(token.kind(), T!['('] | T![')']) {
            count += 1;
        }
        for piece in token.trailing_trivia().pieces() {
            positions.extend(std::iter::repeat_n(count, holes_in(piece.text())));
        }
    }
    positions
}

fn split_pieces(formatted: &str, parts: &[Part], comment_holes: &[bool]) -> Option<Vec<Piece>> {
    let mut pieces = Vec::new();
    let mut cursor = 0;
    for (index, part) in parts.iter().enumerate() {
        if matches!(part, Part::Hole(_)) {
            let placeholder = hole_placeholder(index, comment_holes[index]);
            let found_at = cursor + formatted.get(cursor..)?.find(&placeholder)?;
            if found_at > cursor {
                pieces.push(Piece::Literal(formatted[cursor..found_at].to_string()));
            }
            pieces.push(Piece::Hole(index));
            cursor = found_at + placeholder.len();
        }
    }
    if formatted[cursor..].contains(HOLE_PREFIX) {
        return None;
    }
    if cursor < formatted.len() {
        pieces.push(Piece::Literal(formatted[cursor..].to_string()));
    }
    Some(pieces)
}

impl Format<SqlFormatContext> for EmbeddedQuery {
    fn fmt(&self, f: &mut SqlFormatter) -> FormatResult<()> {
        for part in &self.parts {
            if let Part::Literal(tokens) = part {
                for token in tokens {
                    write!(f, [format_removed(token)])?;
                }
            }
        }
        for operator in &self.operators {
            write!(f, [format_removed(operator)])?;
        }

        let start = self
            .parts
            .iter()
            .find_map(|part| match part {
                Part::Literal(tokens) => tokens
                    .first()
                    .map(|token| token.text_trimmed_range().start()),
                Part::Hole(_) => None,
            })
            .unwrap_or_default();
        let multi_line = self
            .pieces
            .iter()
            .any(|piece| matches!(piece, Piece::Literal(text) if text.contains('\n')));
        let last = self.pieces.len() - 1;

        for (index, piece) in self.pieces.iter().enumerate() {
            if index > 0 {
                write!(f, [text(" || ")])?;
            }
            match piece {
                Piece::Literal(content) => write_literal(
                    content,
                    start,
                    multi_line && index == 0,
                    multi_line && index == last,
                    f,
                )?,
                Piece::Hole(part_index) => {
                    let Part::Hole(expression) = &self.parts[*part_index] else {
                        return Err(FormatError::SyntaxError);
                    };
                    write!(f, [expression.format()])?;
                }
            }
        }
        Ok(())
    }
}

/// `'content'` with `'` escaped; a multi-line query starts on its own line
/// after the opening quote and ends with the closing quote on its own line.
fn write_literal(
    content: &str,
    start: TextSize,
    leading_break: bool,
    trailing_break: bool,
    f: &mut SqlFormatter,
) -> FormatResult<()> {
    write!(
        f,
        [
            text("'"),
            indent(&format_with(|f| {
                if leading_break {
                    write!(f, [hard_line_break()])?;
                }
                for (index, line) in content.split('\n').enumerate() {
                    if index > 0 {
                        write!(f, [hard_line_break()])?;
                    }
                    write!(f, [dynamic_text(&line.replace('\'', "''"), start)])?;
                }
                Ok(())
            }))
        ]
    )?;
    if trailing_break {
        write!(f, [hard_line_break()])?;
    }
    write!(f, [text("'")])
}

/// An `EXECUTE` command: the embedded query when it can be formatted,
/// otherwise the expression as is.
pub(crate) struct FormatExecuteCommand<'a>(
    pub(crate) &'a biome_rowan::SyntaxResult<AnySqlExpression>,
);

impl Format<SqlFormatContext> for FormatExecuteCommand<'_> {
    fn fmt(&self, f: &mut SqlFormatter) -> FormatResult<()> {
        if let Ok(command) = self.0
            && let Some(query) = EmbeddedQuery::try_new(command, f)
        {
            return write!(f, [query]);
        }
        write!(f, [self.0.format()])
    }
}
