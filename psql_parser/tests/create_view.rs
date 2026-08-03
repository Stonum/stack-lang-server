#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_create_view_simple() {
    let res = parse(
        "create view foo as select a from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_or_replace_view() {
    let res = parse(
        "create or replace view foo as select a from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_qualified_name() {
    let res = parse(
        "create view myschema.foo as select a from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_with_options() {
    let res = parse(
        "create view foo with (security_invoker=true) as select a from t;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_with_multiple_options() {
    let res = parse(
        "create view foo with (security_invoker=true, check_option=cascaded) as select a from t",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_complex_select_body() {
    let res = parse(
        "create view foo as select a, b from t where a > 1 order by b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_followed_by_another_statement() {
    let res = parse(
        "create view foo as select a from t; select b from foo;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_tilde_name_in_mlang_dialect() {
    let res = parse("create view ~my_view~ as select a from ~t~", mlang());

    assert_parser!(res);
}

#[test]
fn test_drop_view_bare_name() {
    let res = parse("drop view foo", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_if_exists() {
    let res = parse("drop view if exists foo;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_multiple_names() {
    let res = parse("drop view foo, bar", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_cascade() {
    let res = parse("drop view if exists foo cascade;", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_tilde_name_in_mlang_dialect() {
    let res = parse("drop view if exists ~my_view~;", mlang());

    assert_parser!(res);
}

#[test]
fn test_create_and_drop_view_do_not_shadow_function_table() {
    let res = parse(
        "drop table if exists foo; drop view if exists bar; create table foo (a int); create view bar as select a from foo;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_view_shape() {
    // Representative of a real view: `WITH(security_invoker=true)`,
    // quoted column names in the SELECT body, joins.
    let res = parse(
        r#"create view ~report~
with(security_invoker=true)
as select lf."row_id", lf."Some Column"
from ~source_table~ lf
join ~other_table~ ot on lf.other_id = ot.row_id"#,
        mlang(),
    );

    assert_parser!(res);
}
