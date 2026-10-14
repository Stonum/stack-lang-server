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
fn test_locking_clauses() {
    let src = r#"select * from t for update;
select * from t where a = 1 for no key update nowait;
select * from t for share of t skip locked;
select * from t for key share;
with q as (select * from ~Журналы~ where "Прибор" = 1 for update skip locked) select * from q;
select a from t order by a limit 1 for update into x;"#;

    assert_eq!(count(src, "PSQL_LOCKING_CLAUSE"), 6);
    assert_eq!(count(src, "PSQL_SKIP_LOCKED_POLICY"), 2);
    assert_eq!(count(src, "PSQL_NOWAIT_POLICY"), 1);
    assert_eq!(count(src, "PSQL_LOCKING_OF_CLAUSE"), 1);
}

#[test]
fn test_locking_keywords_are_still_valid_names() {
    let res = parse("select skip, locked, share from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_locking_clauses_report_errors() {
    for src in [
        "select * from t for no update",
        "select * from t for key update",
        "select * from t for update skip",
        "select * from t for update of",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
