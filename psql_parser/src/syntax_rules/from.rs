use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, PsqlExpressionList, count_dotted_name_segments, parse_alias,
    parse_expression, parse_name, parse_shema_qualifier, parse_table_name,
};
use super::parse_error::*;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub(crate) fn parse_from_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![from]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![from]);
    PsqlFromItemList.parse_list(p);
    Present(m.complete(p, PSQL_FROM_CLAUSE))
}

/// A comma-separated list of from-items, shared by `FROM` and `DELETE ...
/// USING` (real Postgres allows the same "implicit cross join" list, each
/// optionally followed by its own `JOIN`s, in both positions).
pub(crate) struct PsqlFromItemList;

impl ParseSeparatedList for PsqlFromItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_FROM_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_from_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![where])
            || p.at(T![group_by])
            || p.at(T![having])
            || p.at(T![order_by])
            || p.at(T![limit])
            || p.at(T![offset])
            || p.at(T![;])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, EXPR_RECOVERY_SET),
            expected_from_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_from_item(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    parse_join_clause_list(p);
    Present(m.complete(p, PSQL_FROM_ITEM))
}

fn parse_join_clause_list(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    while is_at_join_clause(p) {
        let _ = parse_join_clause(p);
    }
    m.complete(p, PSQL_JOIN_CLAUSE_LIST)
}

fn is_at_join_clause(p: &mut PsqlParser) -> bool {
    p.at(T![join]) || p.at(T![inner]) || p.at(T![left]) || p.at(T![right]) || p.at(T![outer])
}

fn parse_join_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !is_at_join_clause(p) {
        return Absent;
    }

    let m = p.start();
    if p.at(T![inner]) || p.at(T![left]) || p.at(T![right]) {
        p.bump_any();
    }
    p.eat(T![outer]);
    p.expect(T![join]);
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    p.expect(T![on]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_JOIN_CLAUSE))
}

/// A table or function binding, e.g. `table`, `schema.table t`, or
/// `some_func(1, 2) as t`. Distinguishes the two by looking ahead past the
/// qualified name for a `(`.
pub(crate) fn parse_from_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    let is_function_call = p.lookahead(|p| {
        for i in 0..segment_count {
            if i > 0 {
                p.bump(T![.]);
            }
            p.bump(T![ident]);
        }
        p.at(T!['('])
    });

    if is_function_call {
        parse_function_binding(p, segment_count)
    } else {
        build_table_binding(p, segment_count)
    }
}

/// A plain table binding, e.g. `table` or `schema.table as t`. Unlike
/// [parse_from_expression], never resolves to a function binding — used by
/// statements whose grammar requires a `PsqlTableBinding` directly (e.g.
/// `DELETE FROM`, `UPDATE`).
pub(crate) fn parse_table_binding(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    build_table_binding(p, segment_count)
}

fn build_table_binding(p: &mut PsqlParser, segment_count: usize) -> ParsedSyntax {
    let m = p.start();
    parse_table_name(p, segment_count);
    parse_alias(p);
    Present(m.complete(p, PSQL_TABLE_BINDING))
}

fn parse_function_binding(p: &mut PsqlParser, segment_count: usize) -> ParsedSyntax {
    let m = p.start();
    parse_shema_qualifier(p, segment_count.saturating_sub(1));
    parse_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T!['(']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![')']);

    parse_alias(p);
    Present(m.complete(p, PSQL_FUNCTION_BINDING))
}
