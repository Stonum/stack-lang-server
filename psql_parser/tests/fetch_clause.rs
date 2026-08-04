#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_fetch_first_rows_only() {
    let res = parse(
        "select a from t fetch first 5 rows only",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_next_row_only() {
    let res = parse(
        "select a from t fetch next 1 row only",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_first_row_no_count() {
    let res = parse(
        "select a from t fetch first row only",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_with_ties() {
    let res = parse(
        "select a from t order by a fetch first 1 rows with ties",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_fetch_after_offset() {
    let res = parse(
        "select a from t offset 5 fetch next 10 rows only",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_without_fetch_clause_still_works() {
    let res = parse("select a from t", PsqlFileSource::script());

    assert_parser!(res);
}
