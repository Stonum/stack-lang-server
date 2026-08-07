#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

#[test]
fn test_double_semicolon_does_not_drop_following_statement() {
    // Regression test: a stray `;` used to be treated as an unrecoverable
    // parse error (it's both the recovery boundary *and* the recovery
    // token, so retrying immediately aborted the whole statement list),
    // silently dropping every statement after it.
    let res = parse("select a from t;; select b from u", SqlFileSource::script());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SELECT_KW").count(), 2);
}

#[test]
fn test_garbage_between_statements_recovers_and_keeps_following_statement() {
    let res = parse(
        "select a from t; garbage here; select b from u",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
    // The tree still builds despite the error -- regression test for
    // `SQL_STATEMENT_LIST`/`SQL_ROOT` silently collapsing into
    // `SQL_BOGUS` when a recovered `SQL_BOGUS_STATEMENT` ended up as a
    // list element before `AnySqlStatement` accepted it.
    assert!(res.try_tree().is_some());
    // Exactly one diagnostic for the garbage -- not a second one for the
    // `;` that follows it.
    assert_eq!(res.diagnostics().len(), 1);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SELECT_KW").count(), 2);
}

#[test]
fn test_triple_semicolon_is_harmless() {
    let res = parse("select a from t;;;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_leading_stray_semicolons_are_harmless() {
    let res = parse(";; select a from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_unclosed_paren_reports_diagnostic_without_panicking() {
    let res = parse("select a from t where (a = 1", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_garbage_input_reports_diagnostic_without_panicking() {
    let res = parse("@#$%", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_go_batch_separator_does_not_swallow_the_next_statement() {
    // `GO` (a T-SQL batch separator some client scripts carry over from
    // MSSQL) isn't given a grammar node -- it falls into ordinary
    // bogus-statement recovery. Recovery must stop as soon as it reaches
    // the start of a real statement, not just at the next `;`, or a
    // perfectly valid statement immediately following `GO` (with no `;` in
    // between) would get swallowed into the same bogus blob and never get
    // its own, properly formattable node.
    let res = parse(
        "drop function if exists foo;\nGO\ncreate function foo() as 'select 1';\nselect 1;",
        SqlFileSource::script().with_dialect(SqlDialect::Postgres),
    );

    assert!(res.has_errors());
    assert!(res.try_tree().is_some());
    let tree = format!("{:#?}", res.syntax());
    // Exactly one diagnostic, for `GO` alone -- not for the `CREATE
    // FUNCTION` statement that follows it.
    assert_eq!(res.diagnostics().len(), 1);
    assert!(tree.contains("PSQL_CREATE_FUNCTION_STATEMENT"));
    assert_eq!(tree.matches("SELECT_KW").count(), 1);
}

#[test]
fn test_lone_semicolon_is_an_empty_statement_not_bogus() {
    let res = parse(";", SqlFileSource::script());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_EMPTY_STATEMENT"));
}
