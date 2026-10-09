//! `CREATE` / `DROP EXTENSION` -- Postgres-only.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::eat_cascade_or_restrict;
use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::SqlTableNameList;
use crate::syntax_rules::expr::parse_name;
use crate::syntax_rules::parse_error::expected_identifier;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `CREATE EXTENSION [IF NOT EXISTS] name [[WITH] SCHEMA schema] [CASCADE] [;]`
pub(crate) fn parse_create_extension_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![create]);
            p.bump(T![extension]);

            if p.at(T![if]) {
                p.bump(T![if]);
                p.expect(T![not]);
                p.expect(T![exists]);
            }

            parse_name(p).or_add_diagnostic(p, expected_identifier);
            let _ = parse_extension_schema_clause(p);
            p.eat(T![cascade]);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_CREATE_EXTENSION_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "Extensions", marker.range(p)),
    )
}

fn parse_extension_schema_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !(p.at(T![schema]) || (p.at(T![with]) && p.nth_at(1, T![schema]))) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![with]);
    p.bump(T![schema]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, PSQL_EXTENSION_SCHEMA_CLAUSE))
}

/// `DROP EXTENSION [IF EXISTS] name, ... [CASCADE | RESTRICT] [;]`
pub(crate) fn parse_drop_extension_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![drop]);
            p.bump(T![extension]);

            if p.at(T![if]) {
                p.bump(T![if]);
                p.expect(T![exists]);
            }

            SqlTableNameList.parse_list(p);
            eat_cascade_or_restrict(p);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_DROP_EXTENSION_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "Extensions", marker.range(p)),
    )
}
