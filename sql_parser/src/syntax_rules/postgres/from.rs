use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::lexer::SqlReLexContext;
use crate::syntax_rules::expr::{
    count_dotted_name_segments, is_at_tilde_name_start, parse_column_name_list,
};
use crate::syntax_rules::from::{parse_function_binding_body, parse_subquery_binding_body};
use crate::syntax_rules::parse_error::expected_column_list;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `JOIN t2 USING (col1, col2)` -- an alternative to `ON`, matching rows by
/// equality on the listed columns instead of an arbitrary condition. T-SQL
/// has no `USING` clause on joins at all (always requires an explicit `ON`)
/// despite this being SQL:1999-standard, not a Postgres invention.
pub(crate) fn parse_join_using_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return ParsedSyntax::Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_join_using_clause_body,
        |p, marker| postgres_only_syntax_error(p, "`JOIN ... USING`", marker.range(p)),
    )
}

fn parse_join_using_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![using]);
    parse_column_name_list(p).or_add_diagnostic(p, expected_column_list);
    Present(m.complete(p, SQL_JOIN_USING_CLAUSE))
}

/// `true` if positioned at `lateral` genuinely followed by a subquery or
/// function call (the only two shapes it can legally precede) -- `false`
/// for a stray `lateral` before anything else, including a plain table
/// name. Doesn't consume anything either way.
pub(crate) fn is_at_lateral_source(p: &mut SqlParser) -> bool {
    p.at(T![lateral])
        && p.lookahead(|p| {
            p.bump(T![lateral]);
            is_at_subquery_or_function_start(p)
        })
}

/// `true` if positioned at the start of a subquery `(` or a (possibly
/// dotted/tilde-wrapped) function call -- the two shapes `lateral` can
/// legally precede. Doesn't consume anything.
fn is_at_subquery_or_function_start(p: &mut SqlParser) -> bool {
    if p.at(T!['(']) {
        return true;
    }
    if is_at_tilde_name_start(p) {
        return p.lookahead(|p| {
            p.re_lex(SqlReLexContext::TildeName) == SQL_TILDE_NAME_LITERAL && {
                p.bump(SQL_TILDE_NAME_LITERAL);
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

/// `lateral` followed by a subquery or function call, e.g. `lateral (select
/// ...) x` / `lateral some_func(t.id) x`. Assumes [is_at_lateral_source]
/// has already confirmed the shape; parses unconditionally (per
/// [SyntaxFeature::parse_exclusive_syntax]) and only turns into a
/// diagnostic + bogus node afterward if the active dialect isn't Postgres.
pub(crate) fn parse_lateral_from_expression(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![lateral]);
            parse_subquery_or_function_binding(p, m)
        },
        |p, marker| postgres_only_syntax_error(p, "`LATERAL`", marker.range(p)),
    )
}

/// Parses whichever of the two `lateral`-eligible shapes actually follows,
/// re-running the same dispatch [is_at_subquery_or_function_start] just
/// confirmed -- `lateral` itself has already been bumped into the
/// still-open marker `m` by the caller, so it ends up as that node's own
/// first child slot.
fn parse_subquery_or_function_binding(p: &mut SqlParser, m: Marker) -> ParsedSyntax {
    if p.at(T!['(']) {
        return parse_subquery_binding_body(p, m);
    }
    if is_at_tilde_name_start(p) {
        return parse_function_binding_body(p, m, 0);
    }
    let segment_count = count_dotted_name_segments(p).min(3);
    parse_function_binding_body(p, m, segment_count)
}
