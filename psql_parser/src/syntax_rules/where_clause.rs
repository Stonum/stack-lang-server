use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::parse_expression;
use super::parse_error::expected_expression;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T};

/// A `where` clause, shared by `SELECT`, `UPDATE` and `DELETE` statements.
pub(crate) fn parse_where_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![where]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![where]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_WHERE_CLAUSE))
}
