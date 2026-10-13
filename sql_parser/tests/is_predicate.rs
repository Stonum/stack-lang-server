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
fn test_is_predicates() {
    for (src, kind) in [
        ("select a is null from t", "SQL_IS_EXPRESSION"),
        ("select a is not null from t", "SQL_IS_EXPRESSION"),
        ("select a IS TRUE from t", "SQL_IS_EXPRESSION"),
        ("select a is not false from t", "SQL_IS_EXPRESSION"),
        ("select a is unknown from t", "SQL_IS_EXPRESSION"),
        (
            "select a is distinct from b from t",
            "SQL_IS_DISTINCT_FROM_EXPRESSION",
        ),
        (
            "select a is not distinct from b + 1 from t",
            "SQL_IS_DISTINCT_FROM_EXPRESSION",
        ),
        ("select a isnull from t", "SQL_POSTFIX_NULL_EXPRESSION"),
        ("select a NOTNULL from t", "SQL_POSTFIX_NULL_EXPRESSION"),
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains(kind), "{src}");
    }
}

#[test]
fn test_is_predicates_in_policy_style_conditions() {
    let res = parse(
        r#"select a from t where "Поле" isnull or "Ключ" is distinct from 1 and uuid_nil() < "Поле" is true"#,
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_unknown_is_still_a_valid_name() {
    let res = parse("select unknown from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_is_without_predicate_reports_error() {
    for src in ["select a is from t", "select a is distinct b from t"] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
