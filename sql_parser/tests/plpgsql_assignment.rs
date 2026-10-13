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
fn test_assignments() {
    let res = parse(
        r#"do $$ declare x int; r record; a int[]; begin
    x := 1;
    x = x + 1;
    r.f := 'a';
    a[1] := (select count(*) from ~Таблица~);
    "Поле" := x = 2;
end $$"#,
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("PSQL_PL_ASSIGNMENT_STATEMENT@").count(), 5);
}

#[test]
fn test_assignment_value_is_a_comparison_after_equals() {
    let res = parse("do $$ begin x = a = b; end $$", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    let assignment = tree.find("PSQL_PL_ASSIGNMENT_STATEMENT@").unwrap();
    assert!(tree[assignment..].contains("SQL_BINARY_EXPRESSION"));
}

#[test]
fn test_sql_statements_are_not_assignments() {
    let res = parse(
        "do $$ begin update t set a = 1; delete from t where a = 1; end $$",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(!tree.contains("PSQL_PL_ASSIGNMENT_STATEMENT"));
}

#[test]
fn test_broken_assignments_report_errors() {
    for src in ["do $$ begin x := ; end $$", "do $$ begin x := 1 end $$"] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
