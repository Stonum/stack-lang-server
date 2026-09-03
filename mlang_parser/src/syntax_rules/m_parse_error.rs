//! Provides factory function to create common diagnostics for the JavaScript syntax

use super::MParser;
use super::span::Span;

use biome_parser::diagnostic::{expected_any, expected_node, expected_token};
use biome_parser::prelude::*;
use biome_rowan::{TextRange, TextSize};
use mlang_syntax::{MSyntaxKind, T};

pub fn expected_function_body(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("function body", range, p)
}

pub fn expected_class_member_name(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub fn expected_object_member(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "shorthand property"], range, p)
}
pub fn expected_array_element(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "expression"], range, p)
}

pub fn expected_block_statement(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block statement", range, p)
}

pub fn expected_catch_clause(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("catch clause", range, p)
}

pub fn expected_parameter(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parameter", range, p)
}

pub fn expected_parameters(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parenthesis '('", range, p)
}

pub fn expected_case_or_default(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["default", "case"], range, p)
}

pub fn expected_case(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("case", range, p)
}

pub fn expected_simple_assignment_target(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "member expression"], range, p)
}

pub fn expected_assignment_target(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("assignment target", range, p)
}

pub fn expected_array_assignment_target_element(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["assignment target", "rest element", "comma"], range, p)
}

pub fn expected_object_assignment_target_property(
    p: &MParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_any(&["assignment target", "rest property"], range, p)
}

pub fn expected_identifier(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub fn expected_statement(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range, p)
}

pub fn expected_binding(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub fn expected_class_member(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property ", "method", "getter", "setter"], range, p)
}

pub fn expected_class_parameters(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class parameters", range, p)
}

pub fn expected_constructor_parameters(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("constructor parameters", range, p)
}

pub fn expected_class_method_body(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class method body", range, p)
}

pub fn expected_expression(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}

pub fn expected_literal_expression(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("literal expression", range, p)
}

pub fn expected_expression_assignment(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["expression", "assignment"], range, p)
}

pub fn expected_declaration(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function", "class", "variable declaration"], range, p)
}

/// Eats the expected `closing` delimiter. On failure, anchors the diagnostic on the
/// *opening* delimiter (`opener`) instead of the current position, so the red range
/// lands on the still-visible construct the user is editing rather than collapsing to
/// a zero-width spot at the end of the file. The point where the closer was expected
/// is kept as a secondary detail label.
///
/// `opener` should be `None` when the opening delimiter itself was missing; in that
/// case the plain "expected token" diagnostic is emitted.
pub fn expect_closing_delimiter(
    p: &mut MParser,
    closing: MSyntaxKind,
    opener: Option<TextRange>,
) -> bool {
    if p.eat(closing) {
        return true;
    }

    let Some(open_range) = opener else {
        p.error(expected_token(closing));
        return false;
    };

    let (open, close) = match closing {
        T!['}'] => ('{', '}'),
        T![')'] => ('(', ')'),
        T![']'] => ('[', ']'),
        _ => {
            p.error(expected_token(closing));
            return false;
        }
    };

    // For `{}` the span is just the opening glyph — `delimiter_balance` widens it
    // to the whole header line afterwards. `()` / `[]` have no such post-pass and
    // never span much, so highlight the entire unclosed group here.
    let primary = if closing == T!['}'] {
        open_range
    } else {
        unclosed_group_range(p, open_range)
    };

    let diagnostic = p
        .err_builder(format!("Missing closing `{close}`"), primary)
        .with_detail(
            p.cur_range(),
            format!("expected `{close}` here to match this `{open}`"),
        )
        .with_hint(format!("this `{open}` is never closed"));
    p.error(diagnostic);

    false
}

/// Range from the opening `(` / `[` to the end of what was actually consumed
/// inside it, stopping before the first non-blank line that dedents back to (or
/// past) the opener's own indentation — that line is no longer part of the group.
fn unclosed_group_range(p: &MParser, opener: TextRange) -> TextRange {
    let src = p.source().text();
    let start = usize::from(opener.start());
    let hard_end = p
        .last_end()
        .map_or(start, usize::from)
        .max(usize::from(opener.end()))
        .min(src.len());

    let line_start = src[..start].rfind('\n').map_or(0, |i| i + 1);
    let opener_indent = indent_width(&src[line_start..start]);

    let mut end = hard_end;
    let mut cursor = start;
    while let Some(rel_nl) = src[cursor..hard_end].find('\n') {
        let nl = cursor + rel_nl;
        let next_line = nl + 1;
        let rest = &src[next_line..];
        let indent = indent_width(rest);
        let is_blank = rest[indent..]
            .bytes()
            .next()
            .is_none_or(|b| b == b'\n' || b == b'\r');
        if !is_blank && indent <= opener_indent {
            end = nl;
            break;
        }
        cursor = next_line;
    }

    TextRange::new(
        opener.start(),
        TextSize::try_from(end).unwrap_or(opener.end()),
    )
}

fn indent_width(line: &str) -> usize {
    line.bytes()
        .take_while(|b| *b == b' ' || *b == b'\t')
        .count()
}

pub fn invalid_assignment_error(p: &MParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid assignment to `{}`", p.text(range.as_range()),),
        range,
    )
    .with_hint("This expression cannot be assigned to")
}
