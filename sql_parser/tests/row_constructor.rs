#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn count(src: &str, kind: &str) -> usize {
    let res = parse(src, postgres());
    assert_parser!(res);
    format!("{:#?}", res.syntax())
        .matches(&format!("{kind}@"))
        .count()
}

#[test]
fn test_row_constructors() {
    let src = r#"select (1, 2);
select row_id from ~Таблица~ where ("Поле", "Родитель") in (select a, b from t);
update t set x = 1 where (a, b) = (1, 2);
select 1 from t where (a, b) in ((1, 2), (3, 4), ((5), 6));"#;

    assert_eq!(count(src, "PSQL_ROW_CONSTRUCTOR_EXPRESSION"), 8);
}

#[test]
fn test_single_parenthesized_expression_is_not_a_row() {
    assert_eq!(
        count("select (1), (a + b) * 2", "PSQL_ROW_CONSTRUCTOR_EXPRESSION"),
        0
    );
}

#[test]
fn test_broken_row_constructors_report_errors() {
    for src in ["select (1, )", "select (1, 2", "select (, 1)"] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
