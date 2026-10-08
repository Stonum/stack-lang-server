use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    count_dotted_name_segments, is_at_name_segment_start, is_at_tilde_name_start,
    parse_call_expression,
};
use super::parse_error::expected_identifier;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T};

/// `CALL name(arg, ...) [;]`
pub(crate) fn parse_call_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![call]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![call]);
    parse_procedure_call(p).or_add_diagnostic(p, expected_identifier);
    p.eat(T![;]);
    Present(m.complete(p, SQL_CALL_STATEMENT))
}

fn parse_procedure_call(p: &mut SqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        return parse_call_expression(p, 0);
    }
    if !is_at_name_segment_start(p) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    parse_call_expression(p, segment_count)
}
