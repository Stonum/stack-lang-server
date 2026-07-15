use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    PsqlExpressionList, count_dotted_name_segments, parse_alias, parse_name, parse_shema_qualifier,
    parse_table_name,
};
use super::parse_error::*;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T};

pub(crate) fn parse_from_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![from]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![from]);
    parse_from_expression(p).or_add_diagnostic(p, expected_from_expression);
    Present(m.complete(p, PSQL_FROM_CLAUSE))
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
