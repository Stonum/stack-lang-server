use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{at_subquery, parse_column_name_list, parse_subquery_expression};
use super::from::parse_table_binding;
use super::parse_error::*;
use super::postgres::on_conflict_clause::parse_on_conflict_clause;
use super::postgres::returning_clause::parse_returning_clause;
use super::select::parse_select_statement;
use super::values::parse_values_clause;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T};

pub(crate) fn parse_insert_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![insert]) {
        return Absent;
    }

    let m = p.start();
    parse_insert_statement_body(p, m)
}

/// Parses the body of an `insert` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `insert_stmt` by the caller.
pub(crate) fn parse_insert_statement_body(p: &mut SqlParser, insert_stmt: Marker) -> ParsedSyntax {
    p.expect(T![insert]);
    p.expect(T![into]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_column_name_list(p);
    parse_insert_source(p).or_add_diagnostic(p, expected_insert_source);
    let _ = parse_on_conflict_clause(p);
    let _ = parse_returning_clause(p);
    p.eat(T![;]);

    Present(insert_stmt.complete(p, SQL_INSERT_STATEMENT))
}

/// The source of the inserted rows: `VALUES (...), (...), ...`, a `SELECT`
/// statement (`INSERT INTO t SELECT ...`), or a parenthesized `SELECT`
/// (`INSERT INTO t (a, b) (SELECT ...)`, real-world confirmed -- redundant
/// parens around the source, distinct from the unparenthesized `SELECT`
/// case only in spelling).
fn parse_insert_source(p: &mut SqlParser) -> ParsedSyntax {
    if at_subquery(p) {
        return parse_subquery_expression(p);
    }

    match p.cur() {
        T![values] => parse_values_clause(p),
        T![select] => parse_select_statement(p),
        _ => Absent,
    }
}
