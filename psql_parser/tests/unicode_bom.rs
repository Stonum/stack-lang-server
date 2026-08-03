#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_leading_bom_is_consumed_without_error() {
    // A Windows-authored script saved as "UTF-8 with BOM" -- real corpus
    // shape.
    let res = parse("\u{FEFF}select a from t", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_leading_bom_before_multiple_statements() {
    let res = parse(
        "\u{FEFF}select a from t;\nselect b from u;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_bom_only_recognized_at_the_very_start() {
    // A stray U+FEFF anywhere else isn't a BOM -- it's just an unexpected
    // character (same as today, unaffected by this fix).
    let res = parse("select a\u{FEFF} from t", PsqlFileSource::script());

    assert!(res.has_errors());
}
