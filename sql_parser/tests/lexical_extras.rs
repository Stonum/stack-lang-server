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
fn test_positional_parameters() {
    let src = r#"do $$
declare x numeric;
begin
    x = $1::numeric;
    execute 'select * from t where a = $1 and b = $2' using $1, $2;
end $$"#;

    assert_eq!(count(src, "PSQL_POSITIONAL_PARAMETER_EXPRESSION"), 3);
}

#[test]
fn test_dollar_quotes_are_still_strings() {
    let src = "select $$a$$, $tag$b$tag$, $1 from t";

    assert_eq!(count(src, "SQL_STRING_LITERAL_EXPRESSION"), 2);
    assert_eq!(count(src, "PSQL_POSITIONAL_PARAMETER_EXPRESSION"), 1);
}

#[test]
fn test_range_overlap_operator() {
    let src = "select daterange(a, b, '[]') && daterange(c, d, '[]'), arr1 && arr2 from t";

    assert_eq!(count(src, "SQL_BINARY_EXPRESSION"), 2);
}

#[test]
fn test_bit_string_constants() {
    let src = "select X'0000FFFF'::int, B'1010', x'ff' from t";

    assert_eq!(count(src, "SQL_STRING_LITERAL_EXPRESSION"), 3);
}

#[test]
fn test_string_continuation_across_lines() {
    let src = "select 'from _flds'\n    ' join t0 on t0.id = _flds.id',\n  'a'\n\n  'b' from t";

    assert_eq!(count(src, "SQL_STRING_LITERAL_EXPRESSION"), 2);
}

#[test]
fn test_strings_on_one_line_do_not_continue() {
    let res = parse("select 'a' 'b' from t", postgres());

    assert!(res.has_errors());
}
