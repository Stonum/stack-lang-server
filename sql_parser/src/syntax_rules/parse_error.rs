//! Provides factory function to create common diagnostics for the JavaScript syntax

use crate::SqlParser;

use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::*;
use biome_rowan::TextRange;

pub fn expected_statement(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range, p)
}

pub fn expected_expression(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}

pub fn expected_identifier(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub fn expected_from_expression(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("table or function binding", range, p)
}

pub fn expected_number_literal(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("number literal", range, p)
}

pub fn expected_limit_value(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("number literal or parameter", range, p)
}

pub fn expected_table_binding(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("table", range, p)
}

pub fn expected_insert_source(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("values or select", range, p)
}

pub fn expected_type_name(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("type name", range, p)
}

pub fn expected_conflict_action(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("do nothing or do update", range, p)
}

pub fn expected_window_specification(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("window specification", range, p)
}

pub fn expected_fetch_tail(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("`only` or `with ties`", range, p)
}

pub fn expected_string_literal(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("string literal", range, p)
}

pub fn expected_column_list(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("column list", range, p)
}

pub fn expected_values_row(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("values row (...)", range, p)
}
