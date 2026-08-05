#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_substring_from_position() {
    let res = parse("select substring(str from 3)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_substring_from_for() {
    let res = parse(
        "select substring(str from 3 for 5)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_substring_from_pattern() {
    // The SQL-standard `from` argument doesn't have to be a start
    // position -- a string literal is a POSIX-regex extract in real
    // Postgres, but syntactically it's just another expression.
    let res = parse(
        r"select substring(col from '^\d+')",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_substring_case_insensitive_keyword() {
    let res = parse(
        "select SUBSTRING(str FROM 3 FOR 5)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_substring_comma_form_still_parses_as_plain_call() {
    // The ordinary comma-separated form isn't the SQL-standard syntax at
    // all -- must still parse as a plain function call, unaffected by the
    // `from`/`for` handling.
    let res = parse("select substring(str, 1, 5)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_substring_single_argument_call_still_parses_as_plain_call() {
    let res = parse("select substring(str)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_realistic_regex_extract_with_cast() {
    let res = parse(
        r"select substring(cfg.value from '^\d+')::int from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_nested_call_argument() {
    let res = parse(
        r"select substring(split_part(sumshare, '/', 1) from '^\d+$')::numeric from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_parenthesized_case_expression_argument() {
    // Representative of the real shape this was fixed for: a `case`
    // expression (wrapped in redundant parens by the original author) as
    // the string argument.
    let res = parse(
        r#"select substring((case when a then b || ', ' else '' end) from 3) as x"#,
        mlang(),
    );

    assert_parser!(res);
}
