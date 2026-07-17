#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_tilde_array_suffix_on_cast_operator() {
    let res = parse("select x::int~[]~ from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_array_suffix_in_real_query_shape() {
    let res = parse(
        "select any(string_to_array('1,2', ',')::int~[]~) from t",
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_tilde_array_suffix_in_cast_function() {
    let res = parse("select cast(x as int~[]~) from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_plain_array_suffix_still_works_in_mlang_dialect() {
    let res = parse("select x::int[] from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_array_suffix_rejected_in_standard_dialect() {
    let res = parse("select x::int~[]~ from t", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_plain_array_suffix_still_works_in_standard_dialect() {
    let res = parse("select x::int[] from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_tilde_operator_after_cast_is_not_misread_as_array_suffix() {
    // Regression test: the position right after a type name can
    // legitimately continue with a plain `~` binary operator (not just the
    // `~[]~` array-suffix escape) -- `is_at_tilde_array_suffix_start` must
    // verify the full 4-token sequence via lookahead before committing,
    // or this would be misparsed as an (invalid, unclosed) array suffix.
    let res = parse("select x::int ~ y from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_operator_with_string_after_cast_is_not_misread() {
    let res = parse("select x::int ~ 'pattern' from t", mlang());

    assert_parser!(res);
}
