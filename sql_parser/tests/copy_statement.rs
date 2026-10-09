#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

fn mlang() -> SqlFileSource {
    postgres().with_mlang_extension(true)
}

#[test]
fn test_copy_statement_forms() {
    for src in [
        "copy t from stdin",
        "COPY \"t\" FROM STDIN (FORMAT CSV, HEADER, delimiter ';', encoding 'UTF8');",
        r"copy t (a, b) from '/tmp/data.txt' with (format text, delimiter E'\b', null '')",
        "copy (select a, b from t where a > 0) to stdout (format csv, header)",
        "copy (with x as (select 1 as a) select a from x) to stdout",
        "copy t to program 'gzip > /tmp/t.gz'",
        "copy s.t to '/tmp/out.csv' (format csv, force_quote *)",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_COPY_STATEMENT"), "{src}");
    }
}

#[test]
fn test_copy_after_create_temp_table_in_mlang_batch() {
    let res = parse(
        "create temp table ~Таблица~ (\"Поле\" text) on commit drop;\ncopy ~Таблица~ from stdin (format csv)\nselect \"Поле\" from ~Таблица~",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_COPY_STATEMENT"));
    assert!(tree.contains("SQL_SELECT_STATEMENT"));
}

#[test]
fn test_copy_keywords_are_still_valid_names() {
    let res = parse("select copy, stdin, stdout, program from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_copy_errors() {
    for src in [
        "copy t stdin",
        "copy t from",
        "copy t from stdin ()",
        "copy from stdin",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_copy_rejected_under_mssql_dialect() {
    let res = parse(
        "copy t from stdin",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}
