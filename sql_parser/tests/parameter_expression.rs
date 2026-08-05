#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Mlang)
}

#[test]
fn test_named_parameter_in_where() {
    let res = parse("select a from t where a = :ls", mlang());

    assert_parser!(res);
}

#[test]
fn test_positional_parameter_in_where() {
    let res = parse("select a from t where a = :1", mlang());

    assert_parser!(res);
}

#[test]
fn test_parameter_as_call_argument() {
    let res = parse("select coalesce(a, :usl) from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_multiple_parameters() {
    let res = parse(
        "select a from t where a = :ls and b = :usl and c = :mes",
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_parameter_in_limit_offset() {
    // LIMIT/OFFSET accept a parameter alongside a bare number literal --
    // real query tools (e.g. paginated queries built with a bind
    // parameter for the page size) rely on this.
    let res = parse("select a from t limit :n offset :o", mlang());

    assert_parser!(res);
}

#[test]
fn test_parameter_in_standard_dialect() {
    // Colon parameters are a common SQL client/tooling convention (DBeaver,
    // JDBC named parameters, SQLAlchemy, ...), not an mlang-specific
    // extension -- available regardless of dialect.
    let res = parse("select a from t where a = :ls", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_colon_colon_cast_still_works_in_mlang_dialect() {
    // `::` (cast) must keep taking priority over `:` (parameter) -- the
    // lexer already glues `::` into one token before the parser ever sees
    // a bare `:`, so this is mostly a regression guard.
    let res = parse("select a::int from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_parameter_name_colliding_with_a_reserved_keyword() {
    // Real-world confirmed: application code names bind parameters
    // independently of this grammar's own reserved words, so a parameter
    // can legitimately be spelled the same as a keyword (`check`, `to`,
    // ...). The `:` prefix already disambiguates unambiguously, so any
    // keyword is accepted here the same way [parse_name] lets a keyword
    // stand in for a name elsewhere.
    let res = parse("select a from t where id = :check", mlang());

    assert_parser!(res);
}

#[test]
fn test_another_parameter_name_colliding_with_a_reserved_keyword() {
    let res = parse("select a from t where id = :to", mlang());

    assert_parser!(res);
}

#[test]
fn test_keyword_named_parameter_in_limit() {
    let res = parse("select a from t limit :key", mlang());

    assert_parser!(res);
}
