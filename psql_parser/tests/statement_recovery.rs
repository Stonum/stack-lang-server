#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_double_semicolon_does_not_drop_following_statement() {
    // Regression test: a stray `;` used to be treated as an unrecoverable
    // parse error (it's both the recovery boundary *and* the recovery
    // token, so retrying immediately aborted the whole statement list),
    // silently dropping every statement after it.
    let res = parse(
        "select a from t;; select b from u",
        PsqlFileSource::script(),
    );

    assert!(!res.has_errors());
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SELECT_KW").count(), 2);
}

#[test]
fn test_garbage_between_statements_recovers_and_keeps_following_statement() {
    let res = parse(
        "select a from t; garbage here; select b from u",
        PsqlFileSource::script(),
    );

    assert!(res.has_errors());
    // Exactly one diagnostic for the garbage -- not a second one for the
    // `;` that follows it.
    assert_eq!(res.diagnostics().len(), 1);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SELECT_KW").count(), 2);
}

#[test]
fn test_triple_semicolon_is_harmless() {
    let res = parse("select a from t;;;", PsqlFileSource::script());

    assert!(!res.has_errors());
}

#[test]
fn test_leading_stray_semicolons_are_harmless() {
    let res = parse(";; select a from t", PsqlFileSource::script());

    assert!(!res.has_errors());
}

#[test]
fn test_unclosed_paren_reports_diagnostic_without_panicking() {
    let res = parse("select a from t where (a = 1", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_garbage_input_reports_diagnostic_without_panicking() {
    let res = parse("@#$%", PsqlFileSource::script());

    assert!(res.has_errors());
}
