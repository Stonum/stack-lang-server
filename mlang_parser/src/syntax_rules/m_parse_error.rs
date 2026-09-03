//! Provides factory function to create common diagnostics for the JavaScript syntax

use super::MParser;
use super::span::Span;

use biome_parser::diagnostic::{expected_any, expected_node, expected_token};
use biome_parser::prelude::*;
use biome_rowan::TextRange;
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

    let diagnostic = p
        .err_builder(format!("Missing closing `{close}`"), open_range)
        .with_detail(
            p.cur_range(),
            format!("expected `{close}` here to match this `{open}`"),
        )
        .with_hint(format!("this `{open}` is never closed"));
    p.error(diagnostic);

    false
}

pub fn invalid_assignment_error(p: &MParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid assignment to `{}`", p.text(range.as_range()),),
        range,
    )
    .with_hint("This expression cannot be assigned to")
}
