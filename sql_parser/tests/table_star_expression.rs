#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::SqlFileSource;

#[test]
fn test_table_star_as_call_argument() {
    let res = parse("select count(t.*) from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_table_star_alongside_other_call_arguments() {
    let res = parse("select f(t.*, 1) from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_schema_qualified_table_star_as_call_argument() {
    let res = parse("select count(s.t.*) from s.t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_table_star_still_works_in_select_item_position() {
    // Regression guard for the select-item shape this now shares its
    // parsing with.
    let res = parse("select t.*, t.a from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_plain_column_reference_still_works() {
    // Regression guard: an ordinary dotted column reference (no trailing
    // `.*`) must still take the normal column-reference/call-expression
    // path, not get misrouted into table-star parsing.
    let res = parse("select t.a from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_bare_star_call_argument_still_works() {
    // Regression guard: an unqualified `*` (e.g. `count(*)`) is a
    // different node (`SqlStar`, no table qualifier) and must be
    // unaffected.
    let res = parse("select count(*) from t", SqlFileSource::script());

    assert_parser!(res);
}
