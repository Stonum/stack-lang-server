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
fn test_nulls_first_and_last() {
    for src in [
        "select a from t order by a nulls last",
        "select a from t order by a desc NULLS FIRST, b asc nulls last",
        "select array_agg(a order by b nulls last) from t",
        "select row_number() over (order by c desc nulls last) from t",
        "create index i on t (a nulls first, b desc nulls last)",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_NULLS_ORDER"), "{src}");
    }
}

#[test]
fn test_nulls_and_last_are_still_valid_names() {
    let res = parse("select nulls, last from t order by last", postgres());

    assert_parser!(res);
}

#[test]
fn test_nulls_without_position_reports_error() {
    let res = parse("select a from t order by a nulls", postgres());

    assert!(res.has_errors());
}
