#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_create_function_no_params_no_returns() {
    let res = parse(
        "create function foo() as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_procedure_no_params() {
    let res = parse(
        "create procedure foo() as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_with_parameters() {
    let res = parse(
        "create function foo(a int, b text) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_scalar() {
    let res = parse(
        "create function foo(a int) returns boolean as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_with_language() {
    let res = parse(
        "create function foo(a int) returns boolean as 'select 1' language plpgsql;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_dollar_quoted_body() {
    let res = parse(
        "create function foo(a int) returns boolean as $$ select 1 $$ language plpgsql;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_tagged_dollar_quoted_body() {
    let res = parse(
        "create function foo(a int) returns boolean as $func$ select 1 $func$ language plpgsql;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_followed_by_another_statement() {
    let res = parse(
        "create function foo() as 'select 1'; select a from t;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_and_drop_function_do_not_shadow_each_other() {
    let res = parse(
        "drop function if exists foo; create function foo() as 'select 1';",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_tilde_name_in_mlang_dialect() {
    let res = parse("create function ~$foo~() as 'select 1'", mlang());

    assert_parser!(res);
}

#[test]
fn test_realistic_scalar_function_shape() {
    // Representative of the simplest real PL/pgSQL functions this grammar
    // is meant to cover -- dollar-quoted body containing quotes/semicolons/
    // newlines untouched, `RETURNS BOOLEAN`, trailing `LANGUAGE plpgsql;`.
    let res = parse(
        r#"create function ~$validate_value~ (value text)
returns boolean as $$
begin
  if length(value) <> 10 and length(value) <> 12 then
    return false;
  end if;
  return true;
end;
$$ language plpgsql;"#,
        mlang(),
    );

    assert_parser!(res);
}
