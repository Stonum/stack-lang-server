use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::expr::{SqlExpressionList, parse_expression};
use crate::syntax_rules::parse_error::expected_expression;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

/// `array[1, 2, 3]` -- Postgres-only, no native array type/literal in
/// T-SQL (including the mlang extension's own `~[...]~` escaped spelling
/// below, which just escapes the same, still-Postgres-only concept).
pub(crate) fn parse_array_expression(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![array]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_array_expression_body,
        |p, marker| postgres_only_syntax_error(p, "Arrays", marker.range(p)),
    )
}

fn parse_array_expression_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![array]);

    // `array` is always a keyword, never a plain column reference, so a
    // `~` immediately after it can only be the mlang dialect's `~[...]~`
    // bracket escaping -- no lookahead needed to disambiguate, unlike
    // `~name~` or the array-type-suffix case.
    if p.source_type().has_mlang_extension() && p.at(T![~]) {
        p.bump(T![~]);
        p.expect(T!['[']);
        SqlExpressionList.parse_list(p);
        p.expect(T![']']);
        p.expect(T![~]);
        return Present(m.complete(p, SQL_TILDE_ARRAY_EXPRESSION));
    }

    p.expect(T!['[']);
    SqlExpressionList.parse_list(p);
    p.expect(T![']']);
    Present(m.complete(p, SQL_ARRAY_EXPRESSION))
}

/// Wraps `expression` in zero or more `[index]` subscripts (e.g. `a[0]`,
/// `matrix[0][1]`). Array subscripting binds tighter than every other
/// operator, so it's applied directly around the primary expression rather
/// than through the binary/logical precedence-climbing chain. Postgres-only
/// (no native array type to subscript in T-SQL) -- gated per-subscript via
/// [SyntaxFeature::exclusive_syntax] on the already-completed node rather
/// than [SyntaxFeature::parse_exclusive_syntax], since this wraps an
/// expression already parsed by the caller rather than parsing from
/// scratch.
pub(crate) fn parse_array_subscript_tail(
    p: &mut SqlParser,
    mut expression: ParsedSyntax,
) -> ParsedSyntax {
    while p.at(T!['[']) {
        if expression.is_absent() {
            break;
        }

        let m = expression.precede(p);
        p.bump(T!['[']);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        p.expect(T![']']);
        let completed = m.complete(p, SQL_ARRAY_SUBSCRIPT_EXPRESSION);
        expression = SqlSyntaxFeature::Postgres.exclusive_syntax(p, completed, |p, marker| {
            postgres_only_syntax_error(p, "Array subscripting", marker.range(p))
        });
    }

    expression
}

/// `int[]` (or, under the mlang extension, its `~[]~`-escaped spelling) --
/// Postgres-only, same reasoning as [parse_array_expression].
pub(crate) fn parse_type_array_suffix(p: &mut SqlParser) -> ParsedSyntax {
    match parse_type_array_suffix_unchecked(p) {
        Present(marker) => SqlSyntaxFeature::Postgres.exclusive_syntax(p, marker, |p, marker| {
            postgres_only_syntax_error(p, "Array types", marker.range(p))
        }),
        Absent => Absent,
    }
}

fn parse_type_array_suffix_unchecked(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T!['[']) {
        let m = p.start();
        p.bump(T!['[']);
        p.expect(T![']']);
        return Present(m.complete(p, SQL_TYPE_ARRAY_SUFFIX));
    }

    if is_at_tilde_array_suffix_start(p) {
        let m = p.start();
        p.bump(T![~]);
        p.expect(T!['[']);
        p.expect(T![']']);
        p.expect(T![~]);
        return Present(m.complete(p, SQL_TILDE_ARRAY_SUFFIX));
    }

    Absent
}

/// `true` if the parser is at the mlang dialect's `~[]~` escaping of the
/// array-type-suffix brackets. Verifies the full 4-token sequence is
/// actually ahead (via lookahead, so nothing is consumed) rather than just
/// checking `p.at(T![~])` -- the position right after a type name can
/// legitimately continue with a plain `~` binary operator too (e.g.
/// `x::int ~ y`), which must not be misread as the start of an array
/// suffix.
fn is_at_tilde_array_suffix_start(p: &mut SqlParser) -> bool {
    if !p.source_type().has_mlang_extension() || !p.at(T![~]) {
        return false;
    }

    p.lookahead(|p| {
        p.bump(T![~]);
        if !p.at(T!['[']) {
            return false;
        }
        p.bump(T!['[']);
        if !p.at(T![']']) {
            return false;
        }
        p.bump(T![']']);
        p.at(T![~])
    })
}

/// `filter (where cond)`, restricting an aggregate call to only the rows
/// matching `cond` instead of the whole group. Postgres-only -- T-SQL has
/// no equivalent (uses `CASE WHEN` inside the aggregate argument instead).
pub(crate) fn parse_filter_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![filter]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(p, parse_filter_clause_body, |p, marker| {
        postgres_only_syntax_error(p, "`FILTER`", marker.range(p))
    })
}

fn parse_filter_clause_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![filter]);
    p.expect(T!['(']);
    p.expect(T![where]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![')']);
    Present(m.complete(p, SQL_FILTER_CLAUSE))
}

/// `true` if the parser is at a `substring(...)` call using the SQL-
/// standard `from`/`for` syntax rather than the ordinary comma-separated
/// call syntax -- i.e. `substring` (case insensitive; not a reserved
/// keyword in Postgres, so recognized by text like `old`/`new` in a
/// trigger's REFERENCING clause) immediately followed by `(`, with a
/// top-level `from` keyword before the matching `)`. A pure token scan
/// (`nth`/`nth_at` never consume), not an actual parse, so it's safe to
/// call speculatively for every ident that happens to be named
/// `substring` -- the ordinary comma-separated form (`substring(string,
/// start, count)`) is left completely alone, still just a plain
/// `SqlCallExpression`.
pub(crate) fn is_at_substring_from_form(p: &mut SqlParser) -> bool {
    if !(p.at(T![ident])
        && p.text(p.cur_range()).eq_ignore_ascii_case("substring")
        && p.nth_at(1, T!['(']))
    {
        return false;
    }

    let mut depth = 0i32;
    let mut i = 1usize;
    loop {
        match p.nth(i) {
            T!['('] | T!['['] => depth += 1,
            T![')'] | T![']'] => {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            T![from] if depth == 1 => return true,
            EOF => return false,
            _ => {}
        }
        i += 1;
    }
}

/// `substring(string from start [for count])` / `substring(string from
/// pattern)`. Only ever reached once [is_at_substring_from_form] has
/// already confirmed the shape (see the primary-expression dispatch), so
/// every `bump`/`expect` below is expected to succeed on well-formed
/// input. ANSI SQL:1999-standard, but T-SQL's `SUBSTRING` is comma-args
/// only -- the `FROM`/`FOR` spelling isn't accepted there.
pub(crate) fn parse_substring_expression(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_substring_expression_body,
        |p, marker| postgres_only_syntax_error(p, "`SUBSTRING(... FROM ...)`", marker.range(p)),
    )
}

fn parse_substring_expression_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![ident]); // `substring`
    p.expect(T!['(']);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    let _ = parse_substring_from_clause(p);
    let _ = parse_substring_for_clause(p);
    p.expect(T![')']);
    Present(m.complete(p, SQL_SUBSTRING_EXPRESSION))
}

fn parse_substring_from_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![from]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![from]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, SQL_SUBSTRING_FROM_CLAUSE))
}

fn parse_substring_for_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![for]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![for]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, SQL_SUBSTRING_FOR_CLAUSE))
}
