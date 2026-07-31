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
fn test_create_function_parameter_mode_in() {
    let res = parse(
        "create function foo(in a int) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_mode_out() {
    let res = parse(
        "create function foo(out a int) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_mode_inout() {
    let res = parse(
        "create function foo(inout a int) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_default() {
    let res = parse(
        "create function foo(a text default '') as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_default_eq_shorthand() {
    // Postgres accepts `= expr` as a shorthand for `DEFAULT expr`; real
    // scripts use both spellings.
    let res = parse(
        "create function foo(a text = '') as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_multiple_parameters_eq_shorthand() {
    let res = parse(
        "create function foo(a text = null, b int = 0) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_default_with_cast() {
    // Real-world shape: an empty-string default cast to the parameter's
    // own type (`DEFAULT ''::type`) is a common pattern for text params.
    let res = parse(
        "create function foo(a varchar default ''::varchar) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_mode_and_default_combined() {
    let res = parse(
        "create function foo(in a text default 'x', out b int) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_table_single_column() {
    let res = parse(
        "create function foo() returns table(a int) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_table_multiple_columns() {
    let res = parse(
        "create function foo() returns table(a int, b text, c boolean) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_table_column_named_full() {
    // Real Postgres doesn't fully reserve `FULL` -- unlike `select`/`from`,
    // it can still be used as a plain column/parameter name (only its
    // meaning inside `FROM ... FULL JOIN` and bare-alias position is
    // special). Real-world shape: a `RETURNS TABLE(...)` column literally
    // named `full`.
    let res = parse(
        "create function foo() returns table(full text, shot text) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_parameter_named_full() {
    let res = parse(
        "create function foo(full text) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_table_typed_column_with_arguments() {
    let res = parse(
        "create function foo() returns table(a numeric(10, 2)) as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_or_replace_function() {
    let res = parse(
        "create or replace function foo() as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_trailing_volatility_option() {
    let res = parse(
        "create function foo() as 'select 1' stable;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_immutable() {
    let res = parse(
        "create function foo() as 'select 1' immutable;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_volatile() {
    let res = parse(
        "create function foo() as 'select 1' volatile;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_security_definer() {
    let res = parse(
        "create function foo() as 'select 1' security definer;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_security_invoker() {
    let res = parse(
        "create function foo() as 'select 1' security invoker;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_strict() {
    let res = parse(
        "create function foo() as 'select 1' strict;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_null_on_null_input_trailing() {
    let res = parse(
        "create function foo() returns text as 'select 1' returns null on null input;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_null_on_null_input_leading() {
    let res = parse(
        "create function foo() returns text returns null on null input as 'select 1';",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_null_on_null_input_with_other_options() {
    let res = parse(
        "create function foo() returns text returns null on null input language plpgsql as 'select 1';",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_leading_options_before_as() {
    let res = parse(
        "create function foo() language plpgsql stable as 'select 1';",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_options_on_both_sides_of_as() {
    // Real-world shape: options appear both before and after `AS`, not just
    // one side -- e.g. `LANGUAGE`+`SECURITY DEFINER` before, `STABLE` after.
    let res = parse(
        "create function foo() language plpgsql security definer as 'select 1' stable;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_multiple_trailing_options_any_order() {
    let res = parse(
        "create function foo() as 'select 1' language plpgsql stable strict;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_trigger() {
    let res = parse(
        "create function foo() returns trigger as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_setof_scalar() {
    let res = parse(
        "create function foo() returns setof int as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_setof_with_trailing_options() {
    let res = parse(
        "create function foo() returns setof int language plpgsql stable as 'select 1';",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_function_returns_setof_named_type() {
    let res = parse(
        "create function foo() returns setof mytype as 'select 1'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_trigger_function_shape() {
    // Representative of a real trigger function: no parameters, `RETURNS
    // TRIGGER`, a body referencing the trigger-context pseudo-tables
    // (`old`/`new`) -- ordinary identifiers to this grammar, no special
    // support needed since the body is opaque.
    let res = parse(
        r#"create function ~$on_update~ ()
returns trigger as $$
declare
  message text = '';
begin
  if (tg_op = 'UPDATE') then
    message = 'updated';
  end if;
  return new;
end;
$$ language plpgsql;"#,
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_table_returning_function_shape() {
    // Representative of a real set-returning function: a wide result-row
    // shape (some quoted column names, since quoted identifiers are common
    // in real mlang table/column names), a `RETURN QUERY SELECT ...` body.
    let res = parse(
        r#"create function ~$build_report~ (root text, period timestamp)
returns table(
  id int,
  "Some Column" varchar(256),
  amount numeric
) as $$
begin
  return query select 1, 'x', 2.5;
end;
$$ language plpgsql;"#,
        mlang(),
    );

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

#[test]
fn test_realistic_procedure_with_mode_and_default_parameters() {
    // Representative of a real procedure signature: multiple `IN`
    // parameters, each with a `DEFAULT` cast to its own type, `LANGUAGE`/
    // `SECURITY DEFINER` options appearing *before* `AS` (a real, common
    // ordering, distinct from the trailing-only options tested elsewhere).
    let res = parse(
        r#"create procedure ~$do_something~ (
  in first_arg varchar default ''::varchar,
  in second_arg varchar default ''::varchar
)
language plpgsql
security definer
as $$
begin
  null;
end;
$$;"#,
        mlang(),
    );

    assert_parser!(res);
}
