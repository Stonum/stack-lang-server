#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_fetch_first_rows_only() {
    let res = parse(
        "select a from t fetch first 5 rows only",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_next_row_only() {
    let res = parse(
        "select a from t fetch next 1 row only",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_first_row_no_count() {
    let res = parse(
        "select a from t fetch first row only",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_with_ties() {
    let res = parse(
        "select a from t order by a fetch first 1 rows with ties",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_after_offset() {
    let res = parse(
        "select a from t offset 5 fetch next 10 rows only",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_without_fetch_clause_still_works() {
    let res = parse("select a from t", SqlFileSource::script());

    assert_parser!(res);
}
