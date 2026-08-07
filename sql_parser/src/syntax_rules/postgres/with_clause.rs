use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `recursive` right after `with` -- Postgres-only (T-SQL has no recursive
/// CTE modifier keyword; a CTE is simply recursive if it references
/// itself). Bumped unconditionally once seen so the parser always makes
/// progress, then diagnosed afterward if the active dialect doesn't
/// support it -- there's no dedicated node to mark bogus for a single
/// modifier keyword, so this reports the diagnostic directly rather than
/// going through [SyntaxFeature::exclusive_syntax].
pub(crate) fn eat_recursive_modifier(p: &mut SqlParser) {
    if !p.at(T![recursive]) {
        return;
    }

    let range = p.cur_range();
    p.bump(T![recursive]);
    if !SqlSyntaxFeature::Postgres.is_supported(p) {
        p.error(postgres_only_syntax_error(p, "`WITH RECURSIVE`", range));
    }
}

/// `[not] materialized`, a purely-informational planner hint on a CTE
/// (`with x as materialized (...)` / `with x as not materialized (...)`)
/// -- doesn't change what the query means, so the parser just accepts and
/// preserves it rather than acting on it. Postgres-only.
pub(crate) fn parse_cte_materialized_hint(p: &mut SqlParser) -> ParsedSyntax {
    if !(p.at(T![materialized]) || p.at(T![not]) && p.nth_at(1, T![materialized])) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_cte_materialized_hint_body,
        |p, marker| postgres_only_syntax_error(p, "`[NOT] MATERIALIZED`", marker.range(p)),
    )
}

fn parse_cte_materialized_hint_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.eat(T![not]);
    p.expect(T![materialized]);
    Present(m.complete(p, PSQL_CTE_MATERIALIZED_HINT))
}
