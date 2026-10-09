use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::parse_table_name_for_ddl;
use super::expr::{EXPR_RECOVERY_SET, is_at_name_start, parse_column_name_list, parse_name};
use super::parse_error::{
    expected_column_list, expected_expression, expected_identifier, expected_table_binding,
};
use super::postgres::parse_error::postgres_only_syntax_error;
use super::select::parse_order_by_expression;
use super::where_clause::parse_where_clause;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T, TextRange};

/// `CREATE [UNIQUE] INDEX [CONCURRENTLY] [[IF NOT EXISTS] name] ON [ONLY] table
/// [USING method] (element, ...) [INCLUDE (column, ...)] [WHERE predicate] [;]`
pub(crate) fn parse_create_index_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);
    p.eat(T![unique]);
    p.expect(T![index]);

    if p.at(T![concurrently]) {
        let range = p.cur_range();
        p.bump(T![concurrently]);
        report_postgres_only(p, "`CONCURRENTLY`", range);
    }

    if p.at(T![if]) {
        let start = p.cur_range().start();
        p.bump(T![if]);
        p.expect(T![not]);
        p.expect(T![exists]);
        let range = TextRange::new(start, p.cur_range().start());
        report_postgres_only(p, "`IF NOT EXISTS`", range);
    }

    if is_at_name_start(p) {
        parse_name(p).ok();
    } else {
        let range = p.cur_range();
        report_postgres_only(p, "An unnamed index", range);
    }

    p.expect(T![on]);
    p.eat(T![only]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_index_method_clause(p);

    p.expect(T!['(']);
    SqlIndexElementList.parse_list(p);
    p.expect(T![')']);

    if p.at(T![include]) {
        let include = p.start();
        p.bump(T![include]);
        parse_column_name_list(p).or_add_diagnostic(p, expected_column_list);
        include.complete(p, SQL_INDEX_INCLUDE_CLAUSE);
    }

    let _ = parse_where_clause(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_CREATE_INDEX_STATEMENT))
}

fn report_postgres_only(p: &mut SqlParser, syntax: &str, range: TextRange) {
    if !SqlSyntaxFeature::Postgres.is_supported(p) {
        p.error(postgres_only_syntax_error(p, syntax, range));
    }
}

/// `USING method`; Postgres-only.
fn parse_index_method_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![using]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            Present(m.complete(p, PSQL_INDEX_METHOD_CLAUSE))
        },
        |p, marker| postgres_only_syntax_error(p, "`USING` index methods", marker.range(p)),
    )
}

struct SqlIndexElementList;

impl ParseSeparatedList for SqlIndexElementList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_INDEX_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_order_by_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}
