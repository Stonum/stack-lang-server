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
fn test_trailing_into() {
    let src = r#"do $$
declare
    _sql text;
begin
    select 'drop index ' || string_agg(indexrelid::regclass::text, ', ')
    from pg_index i
    left join pg_depend d on d.objid = i.indexrelid
    where i.indrelid = 't'::regclass and d.objid is null
    into _sql;
    select a from t order by a limit 1 into strict _sql;
end $$"#;

    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 2);
}

#[test]
fn test_insert_overriding() {
    let src = r#"insert into ~Таблица~ ("Ключ", b) overriding system value select 1, 2;
insert into t overriding user value values (1);"#;

    assert_eq!(count(src, "PSQL_OVERRIDING_CLAUSE"), 2);
}

#[test]
fn test_multi_column_set_item() {
    let src = r#"update ~Таблица~ pp
set ("Начало", "Конец") = (select a, b from s where s.id = pp.row_id), c = 1, (d) = (2)"#;

    assert_eq!(count(src, "PSQL_MULTI_COLUMN_SET_ITEM"), 2);
    assert_eq!(count(src, "SQL_SET_ITEM"), 1);
}

#[test]
fn test_repeated_declare() {
    let src = r#"do $$
declare
    a int = 1;
    declare b varchar(100) = 'x';
    declare
    c refcursor;
begin
end $$"#;

    assert_eq!(count(src, "PSQL_PL_REPEATED_DECLARE"), 2);
    assert_eq!(count(src, "PSQL_PL_VARIABLE_DECLARATION"), 3);
}

#[test]
fn test_new_keywords_are_still_valid_names() {
    let res = parse("select overriding, system, value from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_tail_forms_report_errors() {
    for src in [
        "insert into t overriding value values (1)",
        "insert into t overriding system values (1)",
        "update t set (a, b) (select 1, 2)",
        "update t set (a, b) =",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
