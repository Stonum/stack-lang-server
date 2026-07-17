#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

fn mlang() -> PsqlFileSource {
    PsqlFileSource::script().with_dialect(PsqlDialect::Mlang)
}

#[test]
fn test_tilde_table_name_in_from() {
    let res = parse("select a from ~Договор~", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_table_name_with_spaces() {
    let res = parse("select a from ~Пароли привязка~ pp", mlang());

    assert_parser!(res);
}

#[test]
fn test_dollar_tilde_table_name() {
    let res = parse("select a from ~$ЖурналДействий~ act", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_table_name_in_insert() {
    let res = parse("insert into ~Свойства~ (a) values (1)", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_table_name_in_update() {
    let res = parse("update ~Список объектов~ set a = 1", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_table_name_in_delete() {
    let res = parse("delete from ~Пароли~ where a = 1", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_table_name_in_join() {
    let res = parse("select a from t join ~Договор~ dog on dog.a = t.a", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_table_qualified_column_reference() {
    let res = parse("select a from t where ~Свойства~.row_id = 1", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_function_call() {
    let res = parse("select ~uspSomeFunc~(a, b) from t", mlang());

    assert_parser!(res);
}

#[test]
fn test_tilde_name_rejected_in_standard_dialect() {
    let res = parse("select a from ~Договор~", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_bare_cyrillic_identifier() {
    // Regression test: multi-byte UTF-8 identifier-start bytes (e.g. the
    // first byte of a Cyrillic letter) weren't classified as `IDT` by the
    // byte-level dispatch table (only ASCII is), so they fell through to
    // the generic "unexpected token" error path, which only advances one
    // byte -- corrupting later token boundaries and eventually panicking
    // when the tree-sink tried to slice text at a non-char-boundary index.
    let res = parse("select a from Договор", PsqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_regex_and_like_operators_still_work_in_mlang_dialect() {
    // Regression test: `~~`/`~~*`/`!~~`/`!~~*` (LIKE/ILIKE and their
    // negations) must lex as single compound tokens, not two separate `~`s
    // -- otherwise, in the mlang dialect, the second `~` lands at the start
    // of a new primary expression and gets misread as a tilde-name attempt.
    let res = parse(
        "select a from t where a ~~ 'p' or a ~~* 'p' or a !~~ 'p' or a !~~* 'p' or a ~ 'p' or a ~* 'p' or a !~ 'p' or a !~* 'p'",
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_like_operators_work_in_standard_dialect_too() {
    let res = parse(
        "select a from t where a ~~ 'p' and a ~~* 'p'",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_unclosed_tilde_falls_back_to_operator_without_hanging() {
    // Regression test: a `~` with no matching closing `~` ahead used to
    // make the parser recurse into itself without ever consuming a token
    // (stack overflow) once `is_at_tilde_name_start` naively trusted a bare
    // `~` token as a tilde-name start. It must verify a closing `~` exists
    // (via a speculative re-lex) before committing to that path.
    let res = parse("select a from t where a ~ ~ b", mlang());

    assert!(res.has_errors());
}

#[test]
fn test_unterminated_tilde_name_reports_diagnostic_without_hanging() {
    let res = parse("select ~broken from t", mlang());

    assert!(res.has_errors());
}
