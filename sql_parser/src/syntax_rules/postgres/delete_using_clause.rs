use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::from::SqlFromItemList;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// Postgres-only keyword: T-SQL deletes-via-join instead spell this
/// `DELETE t FROM t JOIN other ON ... WHERE ...`, with no `USING` in its
/// `DELETE` grammar at all.
pub(crate) fn parse_delete_using_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_delete_using_clause_body,
        |p, marker| postgres_only_syntax_error(p, "`DELETE ... USING`", marker.range(p)),
    )
}

fn parse_delete_using_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![using]);
    SqlFromItemList.parse_list(p);
    Present(m.complete(p, PSQL_DELETE_USING_CLAUSE))
}
