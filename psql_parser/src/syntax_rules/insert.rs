use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{PsqlExpressionList, parse_column_name_list, parse_name};
use super::from::parse_table_binding;
use super::parse_error::*;
use super::returning_clause::parse_returning_clause;
use super::select::parse_select_statement;
use super::update::parse_set_clause;
use super::where_clause::parse_where_clause;
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
    let _ = parse_on_conflict_clause(p);
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

/// `ON CONFLICT [target] DO NOTHING | DO UPDATE SET ... [WHERE ...]`,
/// Postgres' upsert clause. The `on` here can only mean `ON CONFLICT`,
/// since it's only reached after the insert source has been fully parsed.
fn parse_on_conflict_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![on]);
    p.expect(T![conflict]);
    let _ = parse_conflict_target(p);
    parse_conflict_action(p).or_add_diagnostic(p, expected_conflict_action);
    Present(m.complete(p, PSQL_ON_CONFLICT_CLAUSE))
}

/// The optional target of a conflict: either a column list (`(col, ...)`)
/// or `ON CONSTRAINT constraint_name`.
fn parse_conflict_target(p: &mut PsqlParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        return parse_column_name_list(p);
    }
    parse_on_constraint_clause(p)
}

fn parse_on_constraint_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![on]);
    p.expect(T![constraint]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, PSQL_ON_CONSTRAINT_CLAUSE))
}

fn parse_conflict_action(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![do]) {
        return Absent;
    }

    let is_do_nothing = p.lookahead(|p| {
        p.bump(T![do]);
        p.at(T![nothing])
    });

    if is_do_nothing {
        parse_do_nothing_clause(p)
    } else {
        parse_do_update_clause(p)
    }
}

fn parse_do_nothing_clause(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![do]);
    p.bump(T![nothing]);
    Present(m.complete(p, PSQL_DO_NOTHING_CLAUSE))
}

fn parse_do_update_clause(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![do]);
    p.expect(T![update]);
    parse_set_clause(p);
    let _ = parse_where_clause(p);
    Present(m.complete(p, PSQL_DO_UPDATE_CLAUSE))
}
