use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, PsqlExpressionList, bump_name_segment, count_dotted_name_segments,
    is_at_name_segment_start, is_at_tilde_name_start, parse_alias, parse_any_name,
    parse_column_name_list, parse_expression, parse_shema_qualifier, parse_table_name,
};
use super::parse_error::*;
use super::with_clause::parse_with_prefixed_select_statement;
use crate::{PsqlParser, lexer::PsqlReLexContext};
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
            || p.at(T![union])
            || p.at(T![intersect])
            || p.at(T![except])
            || p.at(T![order_by])
            || p.at(T![limit])
            || p.at(T![offset])
            || p.at(T![fetch])
            || p.at(T![returning])
            // `insert into t select ... from ... on conflict ...`
            || p.at(T![on])
            || p.at(T![;])
            // A subquery's `from`/`using` list is wrapped in parens, e.g.
            // `(select a from t)`, so it must also stop before `)`.
            || p.at(T![')'])
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

    fn allow_empty(&self) -> bool {
        false
    }
}

fn parse_from_item(p: &mut PsqlParser) -> ParsedSyntax {
    if !is_at_from_expression_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    parse_join_clause_list(p);
    Present(m.complete(p, PSQL_FROM_ITEM))
}

/// `true` if positioned at anything [parse_from_expression] could start
/// consuming -- a plain/dotted/tilde name, `lateral`, or a `(` (subquery,
/// function call, or parenthesized join). Kept as its own function so every
/// "is a from-expression coming next?" gate stays in sync with
/// [parse_from_expression]'s own dispatch, rather than each call site
/// re-deriving (and risking drifting from) the same condition -- see
/// [is_at_lateral_source]'s doc comment for why that drift is a real,
/// previously-hit bug (a stuck-parser panic) and not just tidiness.
fn is_at_from_expression_start(p: &mut PsqlParser) -> bool {
    is_at_name_segment_start(p)
        || p.at(T!['('])
        || is_at_lateral_source(p)
        || is_at_tilde_name_start(p)
}

fn parse_join_clause_list(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    while is_at_join_clause(p) {
        let _ = parse_join_clause(p);
    }
    m.complete(p, PSQL_JOIN_CLAUSE_LIST)
}

fn is_at_join_clause(p: &mut PsqlParser) -> bool {
    p.at(T![join])
        || p.at(T![inner])
        || p.at(T![left])
        || p.at(T![right])
        || p.at(T![full])
        || p.at(T![cross])
        || p.at(T![outer])
}

fn parse_join_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !is_at_join_clause(p) {
        return Absent;
    }

    let m = p.start();
    let is_cross = p.at(T![cross]);
    if p.at(T![inner]) || p.at(T![left]) || p.at(T![right]) || p.at(T![full]) || is_cross {
        p.bump_any();
    }
    p.eat(T![outer]);
    p.expect(T![join]);
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    // `CROSS JOIN` has no `ON`/`USING` clause -- unlike every other join kind.
    if !is_cross {
        if p.at(T![using]) {
            let _ = parse_join_using_clause(p);
        } else {
            p.expect(T![on]);
            parse_expression(p).or_add_diagnostic(p, expected_expression);
        }
    }
    Present(m.complete(p, PSQL_JOIN_CLAUSE))
}

/// `JOIN t2 USING (col1, col2)` -- an alternative to `ON`, matching rows by
/// equality on the listed columns instead of an arbitrary condition.
fn parse_join_using_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![using]);
    parse_column_name_list(p).or_add_diagnostic(p, expected_column_list);
    Present(m.complete(p, PSQL_JOIN_USING_CLAUSE))
}

/// A table binding, function binding, or subquery binding, e.g. `table`,
/// `schema.table t`, `some_func(1, 2) as t`, or `(select ...) as t`.
/// Distinguishes them by looking ahead past the qualified name for a `(`,
/// or by checking for a leading `(select` for subqueries.
pub(crate) fn parse_from_expression(p: &mut PsqlParser) -> ParsedSyntax {
    // `lateral` only ever precedes a subquery or a (possibly
    // dotted/tilde-wrapped) function call in real Postgres -- never a
    // plain table name. [is_at_lateral_source] verifies that via lookahead
    // *before* this commits to consuming it, so a stray `lateral` before
    // anything else falls through to the ordinary dispatch below and fails
    // there with a normal diagnostic instead of this branch eating a token
    // it shouldn't (and, critically, instead of returning `Absent` here
    // without having consumed anything -- see [parse_from_item], which
    // relies on this function always making progress once its own gate,
    // built from the very same check, has committed).
    if is_at_lateral_source(p) {
        let m = p.start();
        p.bump(T![lateral]);
        return parse_subquery_or_function_binding(p, m);
    }

    if p.at(T!['(']) {
        if is_at_parenthesized_join_start(p) {
            return parse_parenthesized_join_binding(p);
        }
        return parse_subquery_binding(p);
    }

    // A tilde name is never schema-qualified, but real mlang scripts do
    // call table-valued functions this way (e.g. `from ~SomeFunc~(:1,
    // :2)`), so still check for a following `(` -- same decision the ident
    // branch below makes, just via a re-lex lookahead since a tilde name
    // needs one to even recognize its own extent.
    if is_at_tilde_name_start(p) {
        let is_function_call = p.lookahead(|p| {
            p.re_lex(PsqlReLexContext::TildeName) == PSQL_TILDE_NAME_LITERAL && {
                p.bump(PSQL_TILDE_NAME_LITERAL);
                p.at(T!['('])
            }
        });

        return if is_function_call {
            parse_function_binding(p, 0)
        } else {
            build_table_binding(p, 0)
        };
    }

    if !is_at_name_segment_start(p) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    let is_function_call = p.lookahead(|p| {
        for i in 0..segment_count {
            if i > 0 {
                p.bump(T![.]);
            }
            bump_name_segment(p);
        }
        p.at(T!['('])
    });

    if is_function_call {
        parse_function_binding(p, segment_count)
    } else {
        build_table_binding(p, segment_count)
    }
}

/// `true` if positioned at `lateral` genuinely followed by a subquery or
/// function call (the only two shapes it can legally precede) -- `false`
/// for a stray `lateral` before anything else, including a plain table
/// name. Doesn't consume anything either way.
fn is_at_lateral_source(p: &mut PsqlParser) -> bool {
    p.at(T![lateral])
        && p.lookahead(|p| {
            p.bump(T![lateral]);
            is_at_subquery_or_function_start(p)
        })
}

/// `true` if positioned at the start of a subquery `(` or a (possibly
/// dotted/tilde-wrapped) function call -- the two shapes `lateral` can
/// legally precede. Doesn't consume anything.
fn is_at_subquery_or_function_start(p: &mut PsqlParser) -> bool {
    if p.at(T!['(']) {
        return true;
    }
    if is_at_tilde_name_start(p) {
        return p.lookahead(|p| {
            p.re_lex(PsqlReLexContext::TildeName) == PSQL_TILDE_NAME_LITERAL && {
                p.bump(PSQL_TILDE_NAME_LITERAL);
                p.at(T!['('])
            }
        });
    }
    if p.at(T![ident]) {
        let segment_count = count_dotted_name_segments(p).min(3);
        return p.lookahead(|p| {
            for i in 0..segment_count {
                if i > 0 {
                    p.bump(T![.]);
                }
                p.bump(T![ident]);
            }
            p.at(T!['('])
        });
    }
    false
}

/// Parses whichever of the two `lateral`-eligible shapes actually follows,
/// re-running the same dispatch [is_at_subquery_or_function_start] just
/// confirmed -- `lateral` itself has already been bumped into the
/// still-open marker `m` by the caller, so it ends up as that node's own
/// first child slot.
fn parse_subquery_or_function_binding(p: &mut PsqlParser, m: Marker) -> ParsedSyntax {
    if p.at(T!['(']) {
        return parse_subquery_binding_body(p, m);
    }
    if is_at_tilde_name_start(p) {
        return parse_function_binding_body(p, m, 0);
    }
    let segment_count = count_dotted_name_segments(p).min(3);
    parse_function_binding_body(p, m, segment_count)
}

/// A plain table binding, e.g. `table` or `schema.table as t`. Unlike
/// [parse_from_expression], never resolves to a function binding — used by
/// statements whose grammar requires a `PsqlTableBinding` directly (e.g.
/// `DELETE FROM`, `UPDATE`).
pub(crate) fn parse_table_binding(p: &mut PsqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        return build_table_binding(p, 0);
    }

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
    parse_function_binding_body(p, m, segment_count)
}

/// Assumes any leading `lateral` has already been bumped into the
/// still-open marker `m` by the caller (see [parse_subquery_or_function_binding]).
fn parse_function_binding_body(
    p: &mut PsqlParser,
    m: Marker,
    segment_count: usize,
) -> ParsedSyntax {
    parse_shema_qualifier(p, segment_count.saturating_sub(1));
    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T!['(']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![')']);

    parse_alias(p);
    Present(m.complete(p, PSQL_FUNCTION_BINDING))
}

/// A subquery used as a table source, e.g. `(select a from t) as sub`.
fn parse_subquery_binding(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    parse_subquery_binding_body(p, m)
}

/// Assumes any leading `lateral` has already been bumped into the
/// still-open marker `m` by the caller (see [parse_subquery_or_function_binding]).
fn parse_subquery_binding_body(p: &mut PsqlParser, m: Marker) -> ParsedSyntax {
    p.bump(T!['(']);
    parse_with_prefixed_select_statement(p).or_add_diagnostic(p, expected_statement);
    p.expect(T![')']);
    parse_alias(p);
    Present(m.complete(p, PSQL_SUBQUERY_BINDING))
}

/// `true` if a `(` is followed by a from-expression rather than `select`/
/// `with` -- i.e. a parenthesized join `(a JOIN b ON ...)` rather than a
/// subquery `(select ...)`. Both start with `(`, so this lookahead is what
/// keeps [parse_from_expression] from routing a parenthesized join into
/// [parse_subquery_binding] (which would then fail expecting `select`).
/// Doesn't consume anything.
fn is_at_parenthesized_join_start(p: &mut PsqlParser) -> bool {
    p.at(T!['(']) && {
        p.lookahead(|p| {
            p.bump(T!['(']);
            !p.at(T![select]) && !p.at(T![with]) && is_at_from_expression_start(p)
        })
    }
}

/// `(a JOIN b ON ...)` used as a from-item, e.g. as the source of an outer
/// join, or just to force a particular join grouping/order. Recursively
/// nestable since `source` is [AnyPsqlFromExpression] itself, which
/// includes this node kind.
fn parse_parenthesized_join_binding(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T!['(']);
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    parse_join_clause_list(p);
    p.expect(T![')']);
    parse_alias(p);
    Present(m.complete(p, PSQL_PARENTHESIZED_JOIN_BINDING))
}
