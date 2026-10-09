//! `CREATE` / `DROP` / `REFRESH MATERIALIZED VIEW` -- Postgres-only.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::{eat_cascade_or_restrict, parse_view_options, parse_with_data_clause};
use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::{
    SqlTableNameList, parse_create_table_as_query, parse_table_name_for_ddl,
};
use crate::syntax_rules::expr::parse_column_name_list;
use crate::syntax_rules::parse_error::{expected_statement, expected_table_binding};
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `CREATE MATERIALIZED VIEW [IF NOT EXISTS] name [(col, ...)] [WITH (...)]
/// AS query [WITH [NO] DATA] [;]`
pub(crate) fn parse_create_materialized_view_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![create]);
            p.bump(T![materialized]);
            p.expect(T![view]);

            if p.at(T![if]) {
                p.bump(T![if]);
                p.expect(T![not]);
                p.expect(T![exists]);
            }

            parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
            let _ = parse_column_name_list(p);
            let _ = parse_view_options(p);

            p.expect(T![as]);
            parse_create_table_as_query(p).or_add_diagnostic(p, expected_statement);
            let _ = parse_with_data_clause(p);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_CREATE_MATERIALIZED_VIEW_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "Materialized views", marker.range(p)),
    )
}

/// `DROP MATERIALIZED VIEW [IF EXISTS] name, ... [CASCADE | RESTRICT] [;]`
pub(crate) fn parse_drop_materialized_view_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![drop]);
            p.bump(T![materialized]);
            p.expect(T![view]);

            if p.at(T![if]) {
                p.bump(T![if]);
                p.expect(T![exists]);
            }

            SqlTableNameList.parse_list(p);
            eat_cascade_or_restrict(p);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_DROP_MATERIALIZED_VIEW_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "Materialized views", marker.range(p)),
    )
}

/// `REFRESH MATERIALIZED VIEW [CONCURRENTLY] name [WITH [NO] DATA] [;]`
pub(crate) fn parse_refresh_materialized_view_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![refresh]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![refresh]);
            p.expect(T![materialized]);
            p.expect(T![view]);
            p.eat(T![concurrently]);

            parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
            let _ = parse_with_data_clause(p);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_REFRESH_MATERIALIZED_VIEW_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "Materialized views", marker.range(p)),
    )
}
