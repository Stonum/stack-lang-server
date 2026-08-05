#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_bracket_identifier_in_select_list() {
    let res = parse("select [Col-Name], [Col2] from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_bracket_identifier_in_where_clause() {
    let res = parse("select a from t where [Col-Name] = :1", mlang());

    assert_parser!(res);
}

#[test]
fn test_dotted_bracket_identifier() {
    let res = parse("select t.[Col Name] from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_bracket_identifier_as_table_qualifier() {
    let res = parse("select a from [Table Name].t", mlang());

    assert_parser!(res);
}

#[test]
fn test_bracket_identifier_in_insert_column_list() {
    let res = parse("insert into t ([Col-Name], [Col2]) values (1, 2)", mlang());

    assert_parser!(res);
}

#[test]
fn test_bracket_identifier_in_update_set_clause() {
    let res = parse("update t set [Col-Name] = 1 where id = 2", mlang());

    assert_parser!(res);
}

#[test]
fn test_bracket_identifier_in_in_list() {
    let res = parse("select a from t where a in ([Col-Name])", mlang());

    assert_parser!(res);
}

#[test]
fn test_array_subscript_still_works_alongside_bracket_identifiers() {
    // Regression guard: `[` as postfix array-subscript punctuation must
    // stay unaffected -- only a `[` at a name-*start* position is ever
    // re-lexed as a bracket-quoted identifier.
    let res = parse("select arr[1] from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_array_type_suffix_still_works_alongside_bracket_identifiers() {
    let res = parse("create table t (a int[])", mlang());

    assert_parser!(res);
}

#[test]
fn test_array_literal_still_works_alongside_bracket_identifiers() {
    let res = parse("select array[1, 2, 3]", mlang());

    assert_parser!(res);
}

#[test]
fn test_bracket_identifier_not_recognized_in_standard_dialect() {
    // Standard Postgres has no `[identifier]` quoting convention -- a bare
    // `[` at an expression start must keep failing there, not silently
    // start accepting SQL-Server-style syntax it never had.
    let res = parse("select [Col] from t", PsqlFileSource::script());

    assert!(res.has_errors());
}
