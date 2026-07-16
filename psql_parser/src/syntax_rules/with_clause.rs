use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{EXPR_RECOVERY_SET, parse_column_name_list, parse_name};
use super::parse_error::*;
use super::select::parse_select_statement_body;
use super::stmt::{StatementContext, parse_statement};
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

/// `with [recursive] cte [(cols)] as (query) [, cte2 ...]`, an optional
/// prefix shared by `SELECT`, `INSERT`, `UPDATE` and `DELETE`.
pub(crate) fn parse_with_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![with]);
    p.eat(T![recursive]);
    PsqlCteDefinitionList.parse_list(p);
    Present(m.complete(p, PSQL_WITH_CLAUSE))
}

/// A `select` statement with an optional leading `with` clause, e.g. the
/// contents of a subquery `(...)`. Unlike a top-level statement (which can
/// dispatch to `select`/`insert`/`update`/`delete` after `with`), a
/// subquery's body can only ever be a `select`, so no dispatch is needed
/// here -- but parsing `with` still can't live inside `select.rs` itself,
/// same as the top-level case in `stmt.rs`.
pub(crate) fn parse_with_prefixed_select_statement(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    let _ = parse_with_clause(p);
    parse_select_statement_body(p, m)
}

struct PsqlCteDefinitionList;

impl ParseSeparatedList for PsqlCteDefinitionList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_CTE_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_cte_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // `is_at_list_end` is checked while still positioned on the `,`
        // separator (before it's consumed), so it must recognize the
        // statement keywords that follow the whole `with` clause rather
        // than "not at an identifier" (which would misfire on `,`).
        p.at(EOF) || p.at(T![select]) || p.at(T![insert]) || p.at(T![update]) || p.at(T![delete])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, EXPR_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_cte_definition(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    let _ = parse_column_name_list(p);
    p.expect(T![as]);
    p.expect(T!['(']);
    // A CTE's body can be any statement, not just `select` -- Postgres
    // allows data-modifying CTEs (`insert`/`update`/`delete ... returning
    // ...`) whose output rows feed into the outer query.
    parse_statement(p, StatementContext::StatementList).or_add_diagnostic(p, expected_statement);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_CTE_DEFINITION))
}
