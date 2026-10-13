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
fn test_collate_expressions() {
    for src in [
        r#"select a collate "C" from t"#,
        r#"select (a || b) collate "und-x-icu" from t"#,
        r#"select a::text collate "C" from t"#,
        r#"select a from t where "Поле" = current_user collate "und-x-icu" limit 1"#,
        r#"select a from t order by a collate "C" desc"#,
        r#"select a from t join u on (u.name::text collate "C") = t.name"#,
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_COLLATE_EXPRESSION"), "{src}");
    }
}

#[test]
fn test_collate_binds_tighter_than_concatenation() {
    let res = parse(r#"select a || b collate "C" from t"#, postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    let binary = tree.find("SQL_BINARY_EXPRESSION").unwrap();
    assert!(tree[binary..].contains("SQL_COLLATE_EXPRESSION"));
}

#[test]
fn test_collate_column_constraint() {
    let res = parse(
        r#"create table t ("Поле" text collate "C" not null, code varchar(10) COLLATE "und-x-icu")"#,
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_COLLATE_CONSTRAINT").count(), 2);
}

#[test]
fn test_collate_without_collation_reports_error() {
    for src in ["select a collate from t", "create table t (a text collate)"] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
