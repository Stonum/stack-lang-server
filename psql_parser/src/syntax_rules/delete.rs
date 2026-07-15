use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::from::{parse_from_expression, parse_table_binding};
use super::parse_error::*;
use super::where_clause::parse_where_clause;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T};

pub(crate) fn parse_delete_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![delete]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![delete]);
    p.expect(T![from]);
    parse_table_binding(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_delete_using_clause(p);
    let _ = parse_where_clause(p);
    p.eat(T![;]);

    Present(m.complete(p, PSQL_DELETE_STATEMENT))
}

fn parse_delete_using_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![using]);
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    Present(m.complete(p, PSQL_DELETE_USING_CLAUSE))
}
