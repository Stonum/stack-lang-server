//! `BEGIN` / `START TRANSACTION` / `COMMIT` / `ROLLBACK`.

use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T};

/// `BEGIN [TRANSACTION | WORK] [;]` / `START TRANSACTION [;]`
pub(crate) fn parse_begin_statement(p: &mut SqlParser) -> ParsedSyntax {
    let is_start = p.at(T![start]);
    if !(p.at(T![begin]) || (is_start && p.nth_at(1, T![transaction]))) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    if is_start {
        p.bump(T![transaction]);
    } else {
        eat_transaction_kind(p);
    }

    p.eat(T![;]);
    Present(m.complete(p, SQL_BEGIN_STATEMENT))
}

/// `COMMIT [TRANSACTION | WORK] [;]`
pub(crate) fn parse_commit_statement(p: &mut SqlParser) -> ParsedSyntax {
    parse_transaction_end(p, T![commit], SQL_COMMIT_STATEMENT)
}

/// `ROLLBACK [TRANSACTION | WORK] [;]`
pub(crate) fn parse_rollback_statement(p: &mut SqlParser) -> ParsedSyntax {
    parse_transaction_end(p, T![rollback], SQL_ROLLBACK_STATEMENT)
}

fn parse_transaction_end(
    p: &mut SqlParser,
    keyword: sql_syntax::SqlSyntaxKind,
    kind: sql_syntax::SqlSyntaxKind,
) -> ParsedSyntax {
    if !p.at(keyword) {
        return Absent;
    }

    let m = p.start();
    p.bump(keyword);
    eat_transaction_kind(p);

    p.eat(T![;]);
    Present(m.complete(p, kind))
}

fn eat_transaction_kind(p: &mut SqlParser) {
    if p.at(T![transaction]) || p.at(T![work]) {
        p.bump_any();
    }
}
