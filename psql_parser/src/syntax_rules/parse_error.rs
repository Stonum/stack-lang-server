//! Provides factory function to create common diagnostics for the JavaScript syntax

use crate::PsqlParser;

use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::*;
use biome_rowan::TextRange;

pub fn expected_statement(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range, p)
}

pub fn expected_expression(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}

pub fn expected_identifier(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub fn expected_from_expression(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("table or function binding", range, p)
}

pub fn expected_number_literal(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("number literal", range, p)
}

pub fn expected_table_binding(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("table", range, p)
}
