use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{PsqlExpressionList, parse_column_name_list};
use super::from::parse_table_binding;
use super::parse_error::*;
use super::returning_clause::parse_returning_clause;
use super::select::parse_select_statement;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T};

pub(crate) fn parse_insert_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![insert]) {
        return Absent;
    }

    let m = p.start();
    parse_insert_statement_body(p, m)
}

/// Parses the body of an `insert` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `insert_stmt` by the caller.
pub(crate) fn parse_insert_statement_body(p: &mut PsqlParser, insert_stmt: Marker) -> ParsedSyntax {
    p.expect(T![insert]);
    p.expect(T![into]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_column_name_list(p);
    parse_insert_source(p).or_add_diagnostic(p, expected_insert_source);
    let _ = parse_returning_clause(p);
    p.eat(T![;]);

    Present(insert_stmt.complete(p, PSQL_INSERT_STATEMENT))
}

/// The source of the inserted rows: either `VALUES (...)` or a `SELECT`
/// statement (`INSERT INTO t SELECT ...`).
fn parse_insert_source(p: &mut PsqlParser) -> ParsedSyntax {
    match p.cur() {
        T![values] => parse_insert_values(p),
        T![select] => parse_select_statement(p),
        _ => Absent,
    }
}

fn parse_insert_values(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![values]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![values]);
    p.expect(T!['(']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_INSERT_VALUES))
}
