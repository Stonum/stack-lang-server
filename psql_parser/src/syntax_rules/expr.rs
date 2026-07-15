use biome_parser::prelude::ParsedSyntax::*;
use biome_parser::prelude::*;

use crate::{
    PsqlParser,
    syntax_rules::parse_error::{expected_expression, expected_identifier},
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
    let left = parse_primary_expression(p);
    parse_binary_or_logical_expression_recursive(p, left, left_precedence)
}

fn parse_binary_or_logical_expression_recursive(
    p: &mut PsqlParser,
    mut left: ParsedSyntax,
    left_precedence: OperatorPrecedence,
) -> ParsedSyntax {
    loop {
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

fn parse_primary_expression(p: &mut PsqlParser) -> ParsedSyntax {
    let literal_expression = parse_literal_expression(p);
    if literal_expression.is_present() {
        return literal_expression;
    }

    match p.cur() {
        T!['('] => parse_parenthesized_expression(p),
        T![ident] => parse_col_reference(p),
        _ => Absent,
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

fn parse_parenthesized_expression(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
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

/// A column reference such as `col`, `table.col`, `schema.table.col` or
/// `db.schema.table.col`. The grammar only supports up to two levels of
/// qualification (schema and database), so longer dotted chains are parsed
/// as `db.schema.table.col` and anything past that is left for the caller
/// to report as an unexpected token.
fn parse_col_reference(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    match count_dotted_name_segments(p) {
        1 => {
            let m = p.start();
            parse_name(p).unwrap();
            Present(m.complete(p, PSQL_COL_REFERENCE))
        }
        2 => {
            let table_col_reference = p.start();
            let table_name = p.start();
            parse_name(p).unwrap();
            table_name.complete(p, PSQL_TABLE_NAME);

            p.bump(T![.]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            Present(table_col_reference.complete(p, PSQL_TABLE_COL_REFERENCE))
        }
        3 => {
            let table_col_reference = p.start();
            let table_name = p.start();
            let schema_name = p.start();
            parse_name(p).unwrap();
            p.bump(T![.]);
            schema_name.complete(p, PSQL_SHEMA_NAME);

            parse_name(p).or_add_diagnostic(p, expected_identifier);
            table_name.complete(p, PSQL_TABLE_NAME);

            p.bump(T![.]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            Present(table_col_reference.complete(p, PSQL_TABLE_COL_REFERENCE))
        }
        _ => {
            let table_col_reference = p.start();
            let table_name = p.start();
            let schema_name = p.start();
            let database_name = p.start();
            parse_name(p).unwrap();
            p.bump(T![.]);
            database_name.complete(p, PSQL_DATA_BASE_NAME);

            parse_name(p).or_add_diagnostic(p, expected_identifier);
            p.bump(T![.]);
            schema_name.complete(p, PSQL_SHEMA_NAME);

            parse_name(p).or_add_diagnostic(p, expected_identifier);
            table_name.complete(p, PSQL_TABLE_NAME);

            p.bump(T![.]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            Present(table_col_reference.complete(p, PSQL_TABLE_COL_REFERENCE))
        }
    }
}

/// Counts the number of `ident (. ident)*` segments ahead without consuming them.
fn count_dotted_name_segments(p: &mut PsqlParser) -> usize {
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

fn parse_name(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![ident]);
    Present(m.complete(p, PSQL_NAME))
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
