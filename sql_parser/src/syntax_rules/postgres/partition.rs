//! Declarative partitioning -- Postgres-only: `PARTITION BY`, `PARTITION OF`,
//! `ATTACH`/`DETACH PARTITION`, plus the `LIKE source` table element.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::parse_table_name_for_ddl;
use crate::syntax_rules::expr::{SqlExpressionList, parse_number_literal_expression};
use crate::syntax_rules::parse_error::{expected_expression, expected_table_binding};
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T, TextRange};

/// `PARTITION BY RANGE | LIST | HASH (key, ...)`
pub(crate) fn parse_partition_by_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![partition_by]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![partition_by]);
            if p.at(T![range]) || p.at(T![list]) || p.at(T![hash]) {
                p.bump_any();
            } else {
                let range = p.cur_range();
                let err = p.err_builder("Expected `range`, `list` or `hash`", range);
                p.error(err);
            }
            p.expect(T!['(']);
            SqlExpressionList.parse_list(p);
            p.expect(T![')']);
            Present(m.complete(p, PSQL_PARTITION_BY_CLAUSE))
        },
        |p, marker| postgres_only_syntax_error(p, "Partitioning", marker.range(p)),
    )
}

/// The rest of `CREATE TABLE name PARTITION OF parent {FOR VALUES ... |
/// DEFAULT} [PARTITION BY ...] [;]`, with `m` started before `create`.
pub(crate) fn complete_create_table_partition_of(p: &mut SqlParser, m: Marker) -> CompletedMarker {
    let start = p.cur_range().start();
    p.bump(T![partition]);
    p.bump(T![of]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
    parse_partition_bound(p).or_add_diagnostic(p, expected_partition_bound);
    let _ = parse_partition_by_clause(p);

    if !SqlSyntaxFeature::Postgres.is_supported(p) {
        let range = TextRange::new(start, p.cur_range().start());
        let err = postgres_only_syntax_error(p, "Partitioning", range);
        p.error(err);
    }

    p.eat(T![;]);
    m.complete(p, PSQL_CREATE_TABLE_PARTITION_OF_STATEMENT)
}

/// `LIKE source_table`
pub(crate) fn parse_table_like_element(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![like]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![like]);
            parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
            Present(m.complete(p, PSQL_TABLE_LIKE_ELEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`LIKE` table elements", marker.range(p)),
    )
}

/// `ATTACH PARTITION name {FOR VALUES ... | DEFAULT}`
pub(crate) fn parse_attach_partition_action(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![attach]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![attach]);
            p.expect(T![partition]);
            parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
            parse_partition_bound(p).or_add_diagnostic(p, expected_partition_bound);
            Present(m.complete(p, PSQL_ATTACH_PARTITION_ACTION))
        },
        |p, marker| postgres_only_syntax_error(p, "Partitioning", marker.range(p)),
    )
}

/// `DETACH PARTITION name [CONCURRENTLY | FINALIZE]`
pub(crate) fn parse_detach_partition_action(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![detach]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![detach]);
            p.expect(T![partition]);
            parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
            if p.at(T![concurrently]) || p.at(T![finalize]) {
                p.bump_any();
            }
            Present(m.complete(p, PSQL_DETACH_PARTITION_ACTION))
        },
        |p, marker| postgres_only_syntax_error(p, "Partitioning", marker.range(p)),
    )
}

/// `DEFAULT` or `FOR VALUES IN (...) | FROM (...) TO (...) | WITH (MODULUS n,
/// REMAINDER r)`
fn parse_partition_bound(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![default]) {
        let m = p.start();
        p.bump(T![default]);
        return Present(m.complete(p, PSQL_PARTITION_DEFAULT_BOUND));
    }
    if !p.at(T![for]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![for]);
    p.expect(T![values]);

    let values = p.start();
    match p.cur() {
        T![in] => {
            p.bump(T![in]);
            p.expect(T!['(']);
            SqlExpressionList.parse_list(p);
            p.expect(T![')']);
            values.complete(p, PSQL_PARTITION_IN_VALUES);
        }
        T![from] => {
            p.bump(T![from]);
            parse_partition_value_list(p);
            p.expect(T![to]);
            parse_partition_value_list(p);
            values.complete(p, PSQL_PARTITION_RANGE_VALUES);
        }
        T![with] => {
            p.bump(T![with]);
            p.expect(T!['(']);
            p.expect(T![modulus]);
            parse_number_literal_expression(p).or_add_diagnostic(p, expected_expression);
            p.expect(T![,]);
            p.expect(T![remainder]);
            parse_number_literal_expression(p).or_add_diagnostic(p, expected_expression);
            p.expect(T![')']);
            values.complete(p, PSQL_PARTITION_HASH_VALUES);
        }
        _ => {
            values.abandon(p);
            let range = p.cur_range();
            let err = p.err_builder("Expected `in`, `from` or `with` after `for values`", range);
            p.error(err);
        }
    }

    Present(m.complete(p, PSQL_PARTITION_FOR_VALUES_BOUND))
}

fn parse_partition_value_list(p: &mut SqlParser) {
    let m = p.start();
    p.expect(T!['(']);
    SqlExpressionList.parse_list(p);
    p.expect(T![')']);
    m.complete(p, PSQL_PARTITION_VALUE_LIST);
}

fn expected_partition_bound(p: &SqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected `for values ...` or `default`", range)
}
