#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_drop_function_bare_name() {
    let res = parse("drop function foo", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_procedure_bare_name() {
    let res = parse("drop procedure foo", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_function_if_exists() {
    let res = parse("drop function if exists foo;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_function_with_empty_parameter_list() {
    let res = parse("drop function foo()", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_function_with_typed_parameters() {
    let res = parse("drop function foo(int, text)", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_function_cascade() {
    let res = parse(
        "drop function if exists foo cascade;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_function_restrict() {
    let res = parse(
        "drop function if exists foo restrict;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_function_followed_by_another_statement() {
    let res = parse(
        "drop function if exists foo; select a from t;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_function_tilde_name_in_mlang_dialect() {
    // Real-world shape (seen in mlang-dialect scripts): `DROP FUNCTION IF
    // EXISTS ~$name~`, no parameter list, no trailing `;` before `GO`.
    let res = parse("drop function if exists ~$do_something~", mlang());

    assert_parser!(res);
}

#[test]
fn test_drop_function_cascade_with_empty_parameters_mlang() {
    // Real-world shape: `drop function if exists ~$name~() cascade;`, with
    // a Cyrillic name (common in real mlang table/function names).
    let res = parse(
        "drop function if exists ~$Значение_ФункцияПодсчета~() cascade;",
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_procedure_semicolon_terminated() {
    // Real-world shape: `DROP PROCEDURE IF EXISTS ~$name~;`
    let res = parse("drop procedure if exists ~$do_something~;", mlang());

    assert_parser!(res);
}
