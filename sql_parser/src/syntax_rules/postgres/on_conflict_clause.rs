use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::expr::{parse_column_name_list, parse_name};
use crate::syntax_rules::parse_error::{expected_conflict_action, expected_identifier};
use crate::syntax_rules::update::parse_set_clause;
use crate::syntax_rules::where_clause::parse_where_clause;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `ON CONFLICT [target] DO NOTHING | DO UPDATE SET ... [WHERE ...]`,
/// Postgres' upsert clause -- T-SQL's upsert is the unrelated `MERGE`
/// statement. The `on` here can only mean `ON CONFLICT`, since it's only
/// reached after the insert source has been fully parsed.
pub(crate) fn parse_on_conflict_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_on_conflict_clause_body,
        |p, marker| postgres_only_syntax_error(p, "`ON CONFLICT`", marker.range(p)),
    )
}

fn parse_on_conflict_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![on]);
    p.expect(T![conflict]);
    let _ = parse_conflict_target(p);
    parse_conflict_action(p).or_add_diagnostic(p, expected_conflict_action);
    Present(m.complete(p, PSQL_ON_CONFLICT_CLAUSE))
}

/// The optional target of a conflict: either a column list (`(col, ...)`)
/// or `ON CONSTRAINT constraint_name`.
fn parse_conflict_target(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        return parse_column_name_list(p);
    }
    parse_on_constraint_clause(p)
}

fn parse_on_constraint_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![on]);
    p.expect(T![constraint]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, PSQL_ON_CONSTRAINT_CLAUSE))
}

fn parse_conflict_action(p: &mut SqlParser) -> ParsedSyntax {
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

fn parse_do_nothing_clause(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![do]);
    p.bump(T![nothing]);
    Present(m.complete(p, PSQL_DO_NOTHING_CLAUSE))
}

fn parse_do_update_clause(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![do]);
    p.expect(T![update]);
    parse_set_clause(p);
    let _ = parse_where_clause(p);
    Present(m.complete(p, PSQL_DO_UPDATE_CLAUSE))
}
