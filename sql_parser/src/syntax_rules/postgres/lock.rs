//! `LOCK [TABLE]` -- Postgres-only.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::SqlTableNameList;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `LOCK [TABLE] [ONLY] name, ... [IN lock_mode MODE] [NOWAIT] [;]`
pub(crate) fn parse_lock_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![lock]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![lock]);
            p.eat(T![table]);
            p.eat(T![only]);

            SqlTableNameList.parse_list(p);
            let _ = parse_lock_mode(p);
            p.eat(T![nowait]);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_LOCK_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`LOCK`", marker.range(p)),
    )
}

/// `IN ACCESS SHARE | ROW SHARE | ROW EXCLUSIVE | SHARE UPDATE EXCLUSIVE
/// | SHARE | SHARE ROW EXCLUSIVE | EXCLUSIVE | ACCESS EXCLUSIVE MODE`
fn parse_lock_mode(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![in]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![in]);

    match p.cur() {
        T![access] | T![row] => {
            p.bump_any();
            if p.at(T![share]) || p.at(T![exclusive]) {
                p.bump_any();
            } else {
                expected_lock_mode(p);
            }
        }
        T![share] => {
            p.bump(T![share]);
            if p.at(T![update]) || p.at(T![row]) {
                p.bump_any();
                p.expect(T![exclusive]);
            }
        }
        T![exclusive] => p.bump(T![exclusive]),
        _ => expected_lock_mode(p),
    }

    p.expect(T![mode]);
    Present(m.complete(p, PSQL_LOCK_MODE))
}

fn expected_lock_mode(p: &mut SqlParser) {
    let range = p.cur_range();
    let err = p.err_builder(
        "Expected a lock mode: `access share`, `row share`, `row exclusive`, `share update exclusive`, `share`, `share row exclusive`, `exclusive` or `access exclusive`",
        range,
    );
    p.error(err);
}
