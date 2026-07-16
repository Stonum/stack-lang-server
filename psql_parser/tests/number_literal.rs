#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_subtraction_without_spaces() {
    // Regression test: the lexer used to greedily swallow `-` into the
    // preceding number, turning `1-2` into a single malformed number
    // token instead of `1`, `-`, `2`.
    let res = parse("select 1-2 from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_addition_without_spaces() {
    let res = parse("select 1+2 from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_exponent_then_subtraction_without_spaces() {
    let res = parse("select 2.5e10-1 from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_malformed_number_reports_diagnostic_without_panicking() {
    // Regression test: a malformed number literal like `1.2.3` used to be
    // silently swallowed into one token by the lexer. Now it correctly
    // lexes as three tokens (`1.2`, `.`, `3`), which used to make the
    // select-item-list parser spin forever at the stray `.` (parser
    // stuck-progress panic) because `parse_select_item` always returned
    // `Present` even when nothing was consumed. Both are fixed; this just
    // checks the parser reports a diagnostic instead of panicking.
    let res = parse("select 1.2.3 from t", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_malformed_number_in_order_by_reports_diagnostic_without_panicking() {
    let res = parse("select a from t order_by 1.2.3", PsqlFileSource::script());

    assert!(res.has_errors());
}
