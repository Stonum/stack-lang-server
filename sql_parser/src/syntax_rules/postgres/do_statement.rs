//! `DO` -- an anonymous code block; Postgres-only.

use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::parse_language_option;
use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::expr::parse_string_literal_expression;
use crate::syntax_rules::parse_error::expected_string_literal;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `DO [LANGUAGE lang] body [LANGUAGE lang] [;]`
pub(crate) fn parse_do_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![do]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![do]);
            let leading = parse_language_option(p);
            parse_string_literal_expression(p).or_add_diagnostic(p, expected_string_literal);
            if leading.is_absent() {
                let _ = parse_language_option(p);
            }

            p.eat(T![;]);
            Present(m.complete(p, PSQL_DO_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`DO`", marker.range(p)),
    )
}
