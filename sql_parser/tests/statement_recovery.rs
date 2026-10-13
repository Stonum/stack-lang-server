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
    // In plain Postgres `GO` is still a `SqlGoStatement`, just with a
    // diagnostic -- the statement right after it (no `;` in between) must
    // keep its own, properly formattable node.
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

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn statement_kinds(res: &sql_parser::Parse<sql_syntax::SqlRoot>) -> Vec<sql_syntax::SqlSyntaxKind> {
    use biome_rowan::{AstNode, AstNodeList};

    res.tree()
        .stmt()
        .iter()
        .map(|stmt| stmt.syntax().kind())
        .collect()
}

#[test]
fn test_broken_select_item_does_not_collapse_the_root() {
    use sql_syntax::SqlSyntaxKind::*;

    // Also a regression test for list recovery running past the `;` and
    // swallowing the following statement into the same bogus node.
    let res = parse("select 1;\nselect a, ? from t;\nselect 2;", postgres());

    assert!(res.has_errors());
    assert!(res.try_tree().is_some());
    assert_eq!(
        statement_kinds(&res),
        [
            SQL_SELECT_STATEMENT,
            SQL_BOGUS_STATEMENT,
            SQL_SELECT_STATEMENT
        ]
    );
}

#[test]
fn test_broken_column_definition_does_not_collapse_the_root() {
    use sql_syntax::SqlSyntaxKind::*;

    let res = parse("create table t (a int, 1);\nselect 1;", postgres());

    assert!(res.has_errors());
    assert!(res.try_tree().is_some());
    assert_eq!(
        statement_kinds(&res),
        [SQL_BOGUS_STATEMENT, SQL_SELECT_STATEMENT]
    );
}

#[test]
fn test_broken_subquery_only_turns_its_own_statement_bogus() {
    use sql_syntax::SqlSyntaxKind::*;

    let res = parse("select (select a, ? from t) from u;\nselect 1;", postgres());

    assert!(res.has_errors());
    assert!(res.try_tree().is_some());
    assert_eq!(
        statement_kinds(&res),
        [SQL_BOGUS_STATEMENT, SQL_SELECT_STATEMENT]
    );
}

#[test]
fn test_lists_end_at_a_reserved_statement_keyword_without_semicolon() {
    use sql_syntax::SqlSyntaxKind::*;

    for (src, second) in [
        (
            "select a, b from t, u\ncreate table x (a int)",
            SQL_CREATE_TABLE_STATEMENT,
        ),
        ("select a, b\nselect c", SQL_SELECT_STATEMENT),
        (
            "select a from t group by a, b\nwith x as (select 1 as a) select a from x",
            SQL_SELECT_STATEMENT,
        ),
        (
            "select a from t order by a, b\ngrant all on t to public",
            SQL_GRANT_STATEMENT,
        ),
        (
            "update t set a = 1, b = 2\nanalyze t",
            PSQL_ANALYZE_STATEMENT,
        ),
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        assert_eq!(statement_kinds(&res).len(), 2, "{src}");
        assert_eq!(statement_kinds(&res)[1], second, "{src}");
    }
}
