//! `DO` -- an anonymous code block; Postgres-only.

use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::parse_language_option;
use super::parse_error::postgres_only_syntax_error;
use super::plpgsql::{is_plpgsql_language, parse_pl_body};
use crate::syntax_rules::expr::parse_string_literal_expression;
use crate::syntax_rules::parse_error::expected_string_literal;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `DO [LANGUAGE lang] body [LANGUAGE lang] [;]` -- a PL/pgSQL body (the
/// default language) is parsed as code, any other one stays a string.
pub(crate) fn parse_do_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![do]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![do]);

            let mut language = language_name_at(p, 0);
            let leading = parse_language_option(p);
            if leading.is_absent() {
                language = language_name_at(p, 1);
            }

            if !is_plpgsql_language(language.as_deref()) || parse_pl_body(p).is_absent() {
                parse_string_literal_expression(p).or_add_diagnostic(p, expected_string_literal);
            }
            if leading.is_absent() {
                let _ = parse_language_option(p);
            }

            p.eat(T![;]);
            Present(m.complete(p, PSQL_DO_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`DO`", marker.range(p)),
    )
}

/// The name of a `LANGUAGE name` clause starting `offset` tokens ahead.
fn language_name_at(p: &mut SqlParser, offset: usize) -> Option<String> {
    p.lookahead(|p| {
        for _ in 0..offset {
            if p.at(EOF) {
                return None;
            }
            p.bump_any();
        }
        if !p.at(T![language]) {
            return None;
        }
        p.bump(T![language]);
        Some(p.text(p.cur_range()).to_string())
    })
}
