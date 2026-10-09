#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

/// `WITH (...)` view storage options are Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_create_view_simple() {
    let res = parse(
        "create view foo as select a from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_or_replace_view() {
    let res = parse(
        "create or replace view foo as select a from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_qualified_name() {
    let res = parse(
        "create view myschema.foo as select a from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_with_options() {
    let res = parse(
        "create view foo with (security_invoker=true) as select a from t;",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_with_multiple_options() {
    let res = parse(
        "create view foo with (security_invoker=true, check_option=cascaded) as select a from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_with_options_rejected_under_standard_dialect() {
    let res = parse(
        "create view foo with (security_invoker=true) as select a from t;",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_create_view_complex_select_body() {
    let res = parse(
        "create view foo as select a, b from t where a > 1 order by b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_view_followed_by_another_statement() {
    let res = parse(
        "create view foo as select a from t; select b from foo;",
        SqlFileSource::script(),
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
    let res = parse("drop view foo", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_if_exists() {
    let res = parse("drop view if exists foo;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_multiple_names() {
    let res = parse("drop view foo, bar", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_view_cascade() {
    let res = parse("drop view if exists foo cascade;", postgres());

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
        SqlFileSource::script(),
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

#[test]
fn test_create_view_query_forms() {
    for src in [
        "create view v as with x as (select 1 as a) select a from x",
        "create or replace view v as with recursive t(n) as (select 1 union all select n + 1 from t where n < 10) select n from t;",
        "create view v as (select a from t)",
        "create view v as (with x as (select 1 as a) select a from x)",
        "create view v as table t",
        "create view v (a, b) as select 1, 2",
        r#"create view ~Представление~ ("Поле", "Ключ") with (security_invoker = true) as with x as (select 1 as a, 2 as b) select a, b from x"#,
    ] {
        let res = parse(src, mlang());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_CREATE_VIEW_STATEMENT"), "{src}");
    }
}

#[test]
fn test_create_view_with_query_followed_by_go_and_grant() {
    let res = parse(
        "create view v as with x as (select 1 as a) select a from x\nGO\ngrant all on table v to public\n",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_GRANT_STATEMENT"));
}

#[test]
fn test_create_view_without_query_reports_error() {
    let res = parse("create view v as;", postgres());

    assert!(res.has_errors());
}
