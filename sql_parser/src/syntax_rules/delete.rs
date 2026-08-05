use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::from::{SqlFromItemList, parse_table_binding};
use super::parse_error::*;
use super::returning_clause::parse_returning_clause;
use super::where_clause::parse_where_clause;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T};

pub(crate) fn parse_delete_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![delete]) {
        return Absent;
    }

    let m = p.start();
    parse_delete_statement_body(p, m)
}

/// Parses the body of a `delete` statement, assuming an optional leading
/// `with` clause has already been parsed (or intentionally omitted) into
/// `delete_stmt` by the caller.
pub(crate) fn parse_delete_statement_body(p: &mut SqlParser, delete_stmt: Marker) -> ParsedSyntax {
    p.expect(T![delete]);
    p.expect(T![from]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_delete_using_clause(p);
    let _ = parse_where_clause(p);
    let _ = parse_returning_clause(p);
    p.eat(T![;]);

    Present(delete_stmt.complete(p, SQL_DELETE_STATEMENT))
}

fn parse_delete_using_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![using]);
    SqlFromItemList.parse_list(p);
    Present(m.complete(p, SQL_DELETE_USING_CLAUSE))
}
