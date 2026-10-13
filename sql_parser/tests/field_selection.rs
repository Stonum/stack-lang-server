#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_field_selection() {
    for src in [
        r#"select (jsonb_each_text(t."Поле")).key::int from t"#,
        "select (jsonb_each_text(t.d)).value as v from t",
        "select (t.c).f.g from t",
        "select (arr)[1].x from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_FIELD_SELECTION_EXPRESSION"), "{src}");
    }
}

#[test]
fn test_field_selection_only_after_parentheses() {
    // `f(x).field` isn't valid Postgres -- the call must be parenthesized.
    let res = parse("select f(x).field from t", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_field_selection_without_name_reports_error() {
    let res = parse("select (t.c). from t", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_field_selection_rejected_under_mssql_dialect() {
    let res = parse(
        "select (t.c).f from t",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}
