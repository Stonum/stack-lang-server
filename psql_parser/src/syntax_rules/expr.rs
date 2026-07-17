use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::*;
use biome_parser::prelude::*;

use super::select::parse_order_by_clause;
use super::with_clause::parse_with_prefixed_select_statement;
use crate::{
    PsqlParser,
    lexer::PsqlReLexContext,
    syntax_rules::parse_error::{
        expected_expression, expected_identifier, expected_number_literal, expected_statement,
        expected_type_name, expected_window_specification,
    },
};
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub const EXPR_RECOVERY_SET: TokenSet<PsqlSyntaxKind> =
    token_set![R_PAREN, L_PAREN, L_BRACK, R_BRACK];

pub(crate) fn parse_expression(p: &mut PsqlParser) -> ParsedSyntax {
    parse_binary_or_logical_expression(p, OperatorPrecedence::lowest())
}

/// A binary expression such as `1 + 2` or a logical expression such as `a and b`
fn parse_binary_or_logical_expression(
    p: &mut PsqlParser,
    left_precedence: OperatorPrecedence,
) -> ParsedSyntax {
    let left = parse_unary_expression(p);
    parse_binary_or_logical_expression_recursive(p, left, left_precedence)
}

fn parse_binary_or_logical_expression_recursive(
    p: &mut PsqlParser,
    mut left: ParsedSyntax,
    left_precedence: OperatorPrecedence,
) -> ParsedSyntax {
    loop {
        if p.at(T![is]) {
            if OperatorPrecedence::IsNull <= left_precedence {
                break;
            }

            if left.is_absent() {
                report_missing_left_operand(p);
            }

            let m = left.precede(p);
            p.bump(T![is]);
            p.eat(T![not]);
            p.expect(T![null]);
            left = Present(m.complete(p, PSQL_IS_NULL_EXPRESSION));
            continue;
        }

        if at_not_prefixed_predicate(p, T![between]) {
            if OperatorPrecedence::Predicate <= left_precedence {
                break;
            }

            if left.is_absent() {
                report_missing_left_operand(p);
            }

            let m = left.precede(p);
            p.eat(T![not]);
            p.bump(T![between]);
            // `and` here is the literal keyword joining `low`/`high`, not a
            // logical continuation, so `low` must stop right before it.
            parse_binary_or_logical_expression(p, OperatorPrecedence::LogicalAnd)
                .or_add_diagnostic(p, expected_expression);
            p.expect(T![and]);
            parse_binary_or_logical_expression(p, OperatorPrecedence::LogicalAnd)
                .or_add_diagnostic(p, expected_expression);
            left = Present(m.complete(p, PSQL_BETWEEN_EXPRESSION));
            continue;
        }

        if at_not_prefixed_predicate(p, T![in]) {
            if OperatorPrecedence::Predicate <= left_precedence {
                break;
            }

            if left.is_absent() {
                report_missing_left_operand(p);
            }

            let m = left.precede(p);
            p.eat(T![not]);
            p.bump(T![in]);
            parse_in_source(p).or_add_diagnostic(p, expected_expression);
            left = Present(m.complete(p, PSQL_IN_EXPRESSION));
            continue;
        }

        let like_op = if at_not_prefixed_predicate(p, T![like]) {
            Some(T![like])
        } else if at_not_prefixed_predicate(p, T![ilike]) {
            Some(T![ilike])
        } else {
            None
        };

        if let Some(like_op) = like_op {
            if OperatorPrecedence::Predicate <= left_precedence {
                break;
            }

            if left.is_absent() {
                report_missing_left_operand(p);
            }

            let m = left.precede(p);
            p.eat(T![not]);
            p.bump(like_op);
            parse_binary_or_logical_expression(p, OperatorPrecedence::Predicate)
                .or_add_diagnostic(p, expected_expression);
            left = Present(m.complete(p, PSQL_LIKE_EXPRESSION));
            continue;
        }

        let op = p.cur();

        let new_precedence = match OperatorPrecedence::try_from_binary_operator(op) {
            Some(precedence) => precedence,
            // Not a binary or logical operator
            None => break,
        };

        if new_precedence <= left_precedence {
            break;
        }

        if left.is_absent() {
            report_missing_left_operand(p);
        }

        let m = left.precede(p);
        p.bump(op);

        parse_binary_or_logical_expression(p, new_precedence)
            .or_add_diagnostic(p, expected_expression);

        let expression_kind = match op {
            T![and] | T![or] => PSQL_LOGICAL_EXPRESSION,
            _ => PSQL_BINARY_EXPRESSION,
        };

        left = Present(m.complete(p, expression_kind));
    }

    left
}

/// Checks whether the parser is at `keyword` or at `not` immediately
/// followed by `keyword` (e.g. `between`/`not between`). `not` can only
/// appear here as a predicate modifier, since a bare `not` is a prefix
/// operator and never valid as an infix continuation token.
fn at_not_prefixed_predicate(p: &mut PsqlParser, keyword: PsqlSyntaxKind) -> bool {
    p.at(keyword)
        || (p.at(T![not])
            && p.lookahead(|p| {
                p.bump(T![not]);
                p.at(keyword)
            }))
}

fn report_missing_left_operand(p: &mut PsqlParser) {
    let op_range = p.cur_range();
    let err = p
        .err_builder(
            format!(
                "Expected an expression for the left hand side of the `{}` operator.",
                p.text(op_range),
            ),
            op_range,
        )
        .with_hint("This operator requires a left hand side value");
    p.error(err);
}

/// A unary expression. `-`/`+` (arithmetic sign) bind tighter than every
/// binary operator, so their operand only recurses into further unary
/// expressions. `not` is different: in SQL it binds looser than comparisons
/// but tighter than `and`/`or`, so its operand recurses into the full
/// binary/logical chain restricted to `OperatorPrecedence::Not` and above.
fn parse_unary_expression(p: &mut PsqlParser) -> ParsedSyntax {
    match p.cur() {
        T![-] | T![+] => {
            let m = p.start();
            p.bump_any();
            parse_unary_expression(p).or_add_diagnostic(p, expected_expression);
            Present(m.complete(p, PSQL_UNARY_EXPRESSION))
        }
        T![not] => {
            let m = p.start();
            p.bump(T![not]);
            parse_binary_or_logical_expression(p, OperatorPrecedence::Not)
                .or_add_diagnostic(p, expected_expression);
            Present(m.complete(p, PSQL_UNARY_EXPRESSION))
        }
        _ => parse_primary_expression(p),
    }
}

fn parse_primary_expression(p: &mut PsqlParser) -> ParsedSyntax {
    let literal_expression = parse_literal_expression(p);
    if literal_expression.is_present() {
        let expression = parse_array_subscript_tail(p, literal_expression);
        return parse_cast_tail(p, expression);
    }

    // A tilde name in expression position is either a call
    // (`~uspSomeFunc~(a, b)`) or a table-qualified column reference
    // (`~Свойства~.row_id`) -- never a bare column-like reference on its
    // own, unlike a plain identifier.
    if is_at_tilde_name_start(p) {
        let reference_or_call = parse_tilde_reference_or_call(p);
        let expression = parse_window_function_tail(p, reference_or_call);
        let expression = parse_array_subscript_tail(p, expression);
        return parse_cast_tail(p, expression);
    }

    let expression = match p.cur() {
        T!['('] => parse_parenthesized_expression(p),
        T![ident] => parse_ident_expression(p),
        T![*] => parse_star(p),
        T![case] => parse_case_expression(p),
        T![array] => parse_array_expression(p),
        T![cast] => parse_cast_function_expression(p),
        _ => Absent,
    };

    let expression = parse_array_subscript_tail(p, expression);
    parse_cast_tail(p, expression)
}

/// `array[1, 2, 3]`.
fn parse_array_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![array]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![array]);
    p.expect(T!['[']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![']']);
    Present(m.complete(p, PSQL_ARRAY_EXPRESSION))
}

/// Wraps `expression` in zero or more `[index]` subscripts (e.g. `a[0]`,
/// `matrix[0][1]`). Array subscripting binds tighter than every other
/// operator, so it's applied directly around the primary expression rather
/// than through the binary/logical precedence-climbing chain.
fn parse_array_subscript_tail(p: &mut PsqlParser, mut expression: ParsedSyntax) -> ParsedSyntax {
    while p.at(T!['[']) {
        if expression.is_absent() {
            break;
        }

        let m = expression.precede(p);
        p.bump(T!['[']);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        p.expect(T![']']);
        expression = Present(m.complete(p, PSQL_ARRAY_SUBSCRIPT_EXPRESSION));
    }

    expression
}

/// `CAST(expr AS type)`, the SQL-standard function-like spelling of a type
/// cast (equivalent to the postfix `expr::type` operator below).
fn parse_cast_function_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![cast]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![cast]);
    p.expect(T!['(']);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![as]);
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_CAST_FUNCTION_EXPRESSION))
}

/// Wraps `expression` in zero or more `::type` casts (e.g. `1::text`,
/// `a::int::text`). Like array subscripting, `::` binds tighter than every
/// other operator, so it's applied directly around the primary expression
/// rather than through the binary/logical precedence-climbing chain.
fn parse_cast_tail(p: &mut PsqlParser, mut expression: ParsedSyntax) -> ParsedSyntax {
    while p.at(T![::]) {
        if expression.is_absent() {
            break;
        }

        let m = expression.precede(p);
        p.bump(T![::]);
        parse_type_name(p).or_add_diagnostic(p, expected_type_name);
        expression = Present(m.complete(p, PSQL_CAST_EXPRESSION));
    }

    expression
}

const TYPE_NAME_TOKEN_SET: TokenSet<PsqlSyntaxKind> = token_set![
    T![ident],
    T![integer],
    T![bigint],
    T![varchar],
    T![char],
    T![text],
    T![boolean],
    T![date],
    T![time],
    T![timestamp],
    T![interval],
    T![numeric],
    T![decimal],
    T![double],
    T![real],
    T![json],
    T![jsonb],
    T![uuid],
    T![array],
    T![bytea],
    T![bit]
];

/// A type name such as `text`, `numeric(10, 2)` or `int[]`, used as the
/// target of a `::` or `CAST(... AS ...)` type cast.
fn parse_type_name(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at_ts(TYPE_NAME_TOKEN_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    let _ = parse_type_arguments(p);
    let _ = parse_type_array_suffix(p);
    Present(m.complete(p, PSQL_TYPE_NAME))
}

fn parse_type_arguments(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    PsqlTypeArgumentList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_TYPE_ARGUMENTS))
}

fn parse_type_array_suffix(p: &mut PsqlParser) -> ParsedSyntax {
    if p.at(T!['[']) {
        let m = p.start();
        p.bump(T!['[']);
        p.expect(T![']']);
        return Present(m.complete(p, PSQL_TYPE_ARRAY_SUFFIX));
    }

    if is_at_tilde_array_suffix_start(p) {
        let m = p.start();
        p.bump(T![~]);
        p.expect(T!['[']);
        p.expect(T![']']);
        p.expect(T![~]);
        return Present(m.complete(p, PSQL_TILDE_ARRAY_SUFFIX));
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
fn is_at_tilde_array_suffix_start(p: &mut PsqlParser) -> bool {
    if !p.source_type().is_mlang_dialect() || !p.at(T![~]) {
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

struct PsqlTypeArgumentList;

impl ParseSeparatedList for PsqlTypeArgumentList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_TYPE_ARGUMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_number_literal_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_number_literal,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// `case [expr] (when cond then result)+ [else default] end`. The optional
/// leading expression is what distinguishes "simple" CASE (`case x when 1
/// then ...`) from "searched" CASE (`case when x = 1 then ...`); both are
/// represented by the same node.
fn parse_case_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![case]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![case]);

    if !p.at(T![when]) {
        parse_expression(p).or_add_diagnostic(p, expected_expression);
    }

    parse_case_when_clause_list(p);
    let _ = parse_case_else_clause(p);
    p.expect(T![end]);

    Present(m.complete(p, PSQL_CASE_EXPRESSION))
}

fn parse_case_when_clause_list(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    while p.at(T![when]) {
        let _ = parse_case_when_clause(p);
    }
    m.complete(p, PSQL_CASE_WHEN_CLAUSE_LIST)
}

fn parse_case_when_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![when]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![when]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![then]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_CASE_WHEN_CLAUSE))
}

fn parse_case_else_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![else]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![else]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_CASE_ELSE_CLAUSE))
}

/// A bare `*`, mainly meant for call arguments like `count(*)`. The grammar
/// allows it anywhere an expression is expected (see `psql.ungram`'s note
/// that ambiguities/precedence are out of scope), matching how `PsqlName`
/// is similarly reachable from `AnyPsqlExpression` without every position
/// making semantic sense.
fn parse_star(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![*]);
    Present(m.complete(p, PSQL_STAR))
}

/// Dispatches a bare identifier to either a call expression (`func(...)`,
/// `schema.func(...)`) or a column reference, by looking ahead past the
/// qualified name for a `(`.
fn parse_ident_expression(p: &mut PsqlParser) -> ParsedSyntax {
    let segment_count = count_dotted_name_segments(p).min(3);
    let is_call = p.lookahead(|p| {
        for i in 0..segment_count {
            if i > 0 {
                p.bump(T![.]);
            }
            p.bump(T![ident]);
        }
        p.at(T!['('])
    });

    if is_call {
        let call = parse_call_expression(p, segment_count);
        parse_window_function_tail(p, call)
    } else {
        parse_col_reference(p)
    }
}

fn parse_call_expression(p: &mut PsqlParser, segment_count: usize) -> ParsedSyntax {
    let m = p.start();
    parse_shema_qualifier(p, segment_count.saturating_sub(1));
    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T!['(']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, PSQL_CALL_EXPRESSION))
}

/// Wraps a call expression in `OVER (...)`, turning it into a window
/// function call (e.g. `row_number() over (partition_by dept order_by
/// salary)`). Only a call can be a window function, so this is applied
/// directly after [parse_call_expression] rather than through the general
/// precedence-climbing chain.
fn parse_window_function_tail(p: &mut PsqlParser, call: ParsedSyntax) -> ParsedSyntax {
    if !p.at(T![over]) || call.is_absent() {
        return call;
    }

    let m = call.precede(p);
    p.bump(T![over]);
    parse_window_specification(p).or_add_diagnostic(p, expected_window_specification);
    Present(m.complete(p, PSQL_WINDOW_FUNCTION_EXPRESSION))
}

fn parse_window_specification(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    let _ = parse_window_partition_by_clause(p);
    let _ = parse_order_by_clause(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_WINDOW_SPECIFICATION))
}

fn parse_window_partition_by_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![partition_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![partition_by]);
    PsqlWindowPartitionByItemList.parse_list(p);
    Present(m.complete(p, PSQL_WINDOW_PARTITION_BY_CLAUSE))
}

struct PsqlWindowPartitionByItemList;

impl ParseSeparatedList for PsqlWindowPartitionByItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_WINDOW_PARTITION_BY_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![order_by]) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

fn parse_literal_expression(p: &mut PsqlParser) -> ParsedSyntax {
    let literal_kind = match p.cur() {
        PSQL_NUMBER_LITERAL => PSQL_NUMBER_LITERAL_EXPRESSION,
        PSQL_STRING_LITERAL => PSQL_STRING_LITERAL_EXPRESSION,
        T![true] | T![false] => PSQL_BOOLEAN_LITERAL_EXPRESSION,
        T![null] => PSQL_NULL_LITERAL_EXPRESSION,
        _ => return Absent,
    };

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, literal_kind))
}

/// A bare number literal, used where the grammar requires a
/// `PsqlNumberLiteralExpression` specifically (e.g. `LIMIT`/`OFFSET`)
/// rather than any general expression.
pub(crate) fn parse_number_literal_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(PSQL_NUMBER_LITERAL) {
        return Absent;
    }
    let m = p.start();
    p.bump(PSQL_NUMBER_LITERAL);
    Present(m.complete(p, PSQL_NUMBER_LITERAL_EXPRESSION))
}

fn parse_parenthesized_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    if at_subquery(p) {
        return parse_subquery_expression(p);
    }

    let m = p.start();
    p.bump(T!('('));

    if p.at(T![')']) {
        // ();
        p.error(
            p.err_builder(
                "Parenthesized expression didn't contain anything",
                p.cur_range(),
            )
            .with_hint("Expected an expression here"),
        );
    } else {
        parse_expression(p).or_add_diagnostic(p, expected_expression);
    }

    p.expect(T![')']);
    Present(m.complete(p, PSQL_PARENTHESIZED_EXPRESSION))
}

/// Checks whether the parser is at `(` immediately followed by `select` or
/// `with` (a `with`-prefixed subquery, e.g. `(with cte as (...) select ...)`),
/// i.e. a subquery rather than a plain parenthesized expression or value list.
fn at_subquery(p: &mut PsqlParser) -> bool {
    p.at(T!['('])
        && p.lookahead(|p| {
            p.bump(T!['(']);
            p.at(T![select]) || p.at(T![with])
        })
}

fn parse_subquery_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !at_subquery(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    parse_with_prefixed_select_statement(p).or_add_diagnostic(p, expected_statement);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_SUBQUERY_EXPRESSION))
}

/// The source of an `in` predicate: either a parenthesized value list
/// (`in (1, 2, 3)`) or a subquery (`in (select ...)`).
fn parse_in_source(p: &mut PsqlParser) -> ParsedSyntax {
    if at_subquery(p) {
        return parse_subquery_expression(p);
    }

    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    PsqlExpressionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_IN_VALUE_LIST))
}

/// A column reference such as `col`, `table.col`, `schema.table.col` or
/// `db.schema.table.col`. The grammar only supports up to two levels of
/// qualification (schema and database), so longer dotted chains are parsed
/// as `db.schema.table.col` and anything past that is left for the caller
/// to report as an unexpected token.
fn parse_col_reference(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let total_segments = count_dotted_name_segments(p);
    if total_segments <= 1 {
        let m = p.start();
        parse_name(p).unwrap();
        return Present(m.complete(p, PSQL_COL_REFERENCE));
    }

    let table_col_reference = p.start();
    parse_table_name(p, (total_segments - 1).min(3));

    p.bump(T![.]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(table_col_reference.complete(p, PSQL_TABLE_COL_REFERENCE))
}

/// A tilde name in expression position: `~name~.col` (table-qualified
/// column reference, e.g. `~Свойства~.row_id`) if a `.` follows, otherwise
/// `~name~(...)` (call expression). Mirrors how a plain identifier's `.`
/// vs `(` decides between [parse_col_reference] and [parse_call_expression],
/// but a tilde name is never a *bare* column-like reference on its own.
fn parse_tilde_reference_or_call(p: &mut PsqlParser) -> ParsedSyntax {
    let is_table_col_reference = p.lookahead(|p| {
        p.re_lex(PsqlReLexContext::TildeName) == PSQL_TILDE_NAME_LITERAL && {
            p.bump(PSQL_TILDE_NAME_LITERAL);
            p.at(T![.])
        }
    });

    if is_table_col_reference {
        let table_col_reference = p.start();
        parse_table_name(p, 0);
        p.bump(T![.]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        Present(table_col_reference.complete(p, PSQL_TABLE_COL_REFERENCE))
    } else {
        parse_call_expression(p, 0)
    }
}

/// Parses a possibly schema/database-qualified table name: `table`,
/// `schema.table` or `db.schema.table` (`segment_count` = 1, 2 or 3).
/// Assumes the parser is currently at an `ident` and that at least
/// `segment_count` dotted segments are ahead.
pub(crate) fn parse_table_name(p: &mut PsqlParser, segment_count: usize) -> CompletedMarker {
    let table_name = p.start();
    parse_shema_qualifier(p, segment_count.saturating_sub(1));
    parse_any_name(p).or_add_diagnostic(p, expected_identifier);
    table_name.complete(p, PSQL_TABLE_NAME)
}

/// Parses the leading `qualifier_count` dotted segments of a qualified name
/// as an optional schema qualifier: 0 = nothing, 1 = `schema.`, 2 =
/// `db.schema.`. Leaves the parser positioned at the final, unqualified name.
pub(crate) fn parse_shema_qualifier(p: &mut PsqlParser, qualifier_count: usize) {
    match qualifier_count {
        0 => {}
        1 => {
            let schema_name = p.start();
            parse_name(p).unwrap();
            p.bump(T![.]);
            schema_name.complete(p, PSQL_SHEMA_NAME);
        }
        _ => {
            let schema_name = p.start();
            let database_name = p.start();
            parse_name(p).unwrap();
            p.bump(T![.]);
            database_name.complete(p, PSQL_DATA_BASE_NAME);

            parse_name(p).or_add_diagnostic(p, expected_identifier);
            p.bump(T![.]);
            schema_name.complete(p, PSQL_SHEMA_NAME);
        }
    }
}

/// Counts the number of `ident (. ident)*` segments ahead without consuming them.
pub(crate) fn count_dotted_name_segments(p: &mut PsqlParser) -> usize {
    p.lookahead(|p| {
        let mut count = 0;
        while p.at(T![ident]) {
            count += 1;
            p.bump(T![ident]);
            if !p.at(T![.]) {
                break;
            }
            p.bump(T![.]);
        }
        count
    })
}

pub(crate) fn parse_name(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![ident]);
    Present(m.complete(p, PSQL_NAME))
}

/// `true` if the parser is at a `~` that could be the start of a
/// mlang-dialect tilde name -- i.e. only in the mlang dialect at all. A bare
/// `~` never legitimately starts a table/function name/primary expression
/// in standard Postgres, so this can't misfire there. Actually attempts the
/// re-lex (inside a lookahead, so nothing is consumed) rather than just
/// checking `p.at(T![~])`: a `~` with no matching closing `~` ahead (e.g. a
/// lone regex-operator `~` with nothing after it) must *not* count as a
/// tilde-name start, or callers that commit to the tilde-name path on the
/// strength of this check alone would recurse back into themselves without
/// ever consuming a token, since nothing else stops them from retrying.
pub(crate) fn is_at_tilde_name_start(p: &mut PsqlParser) -> bool {
    if !p.source_type().is_mlang_dialect() || !p.at(T![~]) {
        return false;
    }

    p.lookahead(|p| p.re_lex(PsqlReLexContext::TildeName) == PSQL_TILDE_NAME_LITERAL)
}

/// A `PsqlName` (`ident`) or, in the mlang dialect, a `~name~`/`~$name~`
/// tilde name -- used specifically in table/function-name position, never
/// for columns/aliases/CTEs (see `PsqlReLexContext::TildeName`).
pub(crate) fn parse_any_name(p: &mut PsqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        let kind = p.re_lex(PsqlReLexContext::TildeName);
        if kind == PSQL_TILDE_NAME_LITERAL {
            let m = p.start();
            p.bump(PSQL_TILDE_NAME_LITERAL);
            return Present(m.complete(p, PSQL_TILDE_NAME));
        }
    }

    parse_name(p)
}

pub(crate) fn parse_alias(p: &mut PsqlParser) {
    if !p.at(T![as]) && !p.at(T![ident]) {
        return;
    }

    let m = p.start();
    if p.at(T![as]) {
        p.bump(T![as]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
    } else {
        parse_name(p).unwrap();
    }
    m.complete(p, PSQL_ALIAS);
}

/// A parenthesized, comma-separated expression list with no trailing comma,
/// e.g. function call arguments or `VALUES (...)` items.
pub(crate) struct PsqlExpressionList;

impl ParseSeparatedList for PsqlExpressionList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_EXPRESSION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // `)` closes call/values/in-list arguments; `]` closes an
        // `array[...]` literal's items.
        p.at(EOF) || p.at(T![')']) || p.at(T![']'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// A parenthesized, comma-separated list of plain column names, e.g.
/// `(a, b, c)` — shared by `INSERT`'s target columns and a CTE's column
/// aliases.
pub(crate) fn parse_column_name_list(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    PsqlColumnNameList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_COLUMN_LIST))
}

struct PsqlColumnNameList;

impl ParseSeparatedList for PsqlColumnNameList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_COLUMN_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, EXPR_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}
