use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::expr::{SqlExpressionList, parse_limit_offset_value};
use crate::syntax_rules::parse_error::expected_limit_value;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// Postgres-only extension -- T-SQL has plain `DISTINCT` only, no `ON`
/// column-subset spelling.
pub(crate) fn parse_distinct_on_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_distinct_on_clause_body,
        |p, marker| postgres_only_syntax_error(p, "`DISTINCT ON`", marker.range(p)),
    )
}

fn parse_distinct_on_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![on]);
    p.expect(T!['(']);
    SqlExpressionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, SQL_DISTINCT_ON_CLAUSE))
}

/// Postgres/MySQL-style row-limiting -- T-SQL has no `LIMIT` keyword at all
/// (uses `TOP`, or the SQL-standard `FETCH FIRST/NEXT ... ROWS ONLY`
/// clause, which is already shared -- see `parse_fetch_clause`). `OFFSET`
/// is deliberately left ungated: T-SQL does support it, just always paired
/// with a mandatory following `FETCH NEXT`, a distinction this grammar
/// doesn't currently track finely enough to gate just the bare
/// (fetch-less) spelling without also rejecting the legitimately-shared
/// form.
pub(crate) fn parse_limit_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![limit]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(p, parse_limit_clause_body, |p, marker| {
        postgres_only_syntax_error(p, "`LIMIT`", marker.range(p))
    })
}

fn parse_limit_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![limit]);
    parse_limit_offset_value(p).or_add_diagnostic(p, expected_limit_value);
    Present(m.complete(p, SQL_LIMIT_CLAUSE))
}
