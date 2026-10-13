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
fn test_foreach_loops() {
    let src = r#"do $$
declare
    name text;
BEGIN
    FOREACH name IN ARRAY regexp_split_to_array("Поле", ' ') LOOP
        delete from ~Таблица~ where "Ключ" = name;
    END LOOP;
    foreach field in array fieldUpdates loop end loop;
    FOREACH tblname IN ARRAY ARRAY(SELECT relname FROM pg_class) LOOP END LOOP;
    <<rows>>
    foreach row_value slice 1 in array matrix loop
        exit rows;
    end loop rows;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_FOREACH_STATEMENT"), 4);
    assert_eq!(count(src, "PSQL_PL_FOREACH_SLICE"), 1);
}

#[test]
fn test_foreach_keywords_are_still_valid_names() {
    let res = parse("select foreach, slice from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_foreach_reports_errors() {
    for src in [
        "do $$ begin foreach x in arr loop end loop; end $$",
        "do $$ begin foreach x array arr loop end loop; end $$",
        "do $$ begin foreach x in array loop end loop; end $$",
        "do $$ begin foreach x slice in array arr loop end loop; end $$",
        "do $$ begin foreach x in array arr end loop; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_array_subquery_expression() {
    let src = "select array(select a from t), array[1, 2] from t";

    assert_eq!(count(src, "PSQL_ARRAY_SUBQUERY_EXPRESSION"), 1);
    assert_eq!(count(src, "PSQL_ARRAY_EXPRESSION"), 1);
}
