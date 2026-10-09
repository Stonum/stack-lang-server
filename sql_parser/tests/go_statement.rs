#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

fn mlang() -> SqlFileSource {
    postgres().with_mlang_extension(true)
}

#[test]
fn test_go_separates_batches_in_mlang_scripts() {
    let res = parse(
        r#"drop view if exists ~Представление~
GO
create view ~Представление~ as select a from t
go
GRANT ALL ON TABLE ~Представление~ TO public;
Go
"#,
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_GO_STATEMENT").count(), 3);
    assert!(tree.contains("SQL_CREATE_VIEW_STATEMENT"));
    assert!(tree.contains("SQL_GRANT_STATEMENT"));
}

#[test]
fn test_go_ends_select_from_and_update_lists() {
    for src in [
        "select a, b\nGO",
        "select a from t, u\nGO",
        "select a from t group by a, b\nGO",
        "select a from t order by a, b\nGO",
        "update t set a = 1, b = 2\nGO",
        "truncate a, b\nGO",
        "create function f() returns int as $$ select 1 $$ language sql;\nGO",
    ] {
        let res = parse(src, mlang());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_GO_STATEMENT"), "{src}");
    }
}

#[test]
fn test_go_at_start_of_file_and_with_semicolon() {
    let res = parse("GO\nselect 1\nGO;\n", mlang());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_GO_STATEMENT").count(), 2);
}

#[test]
fn test_go_is_still_a_valid_name() {
    let res = parse("select go, a.go from go as a", mlang());

    assert_parser!(res);
}

#[test]
fn test_go_not_on_its_own_line_reports_error() {
    for src in ["select 1; GO", "select 1\nGO select 2"] {
        let res = parse(src, mlang());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_go_rejected_in_plain_postgres_but_parsing_continues() {
    let res = parse("select 1\nGO\nselect 2", postgres());

    assert!(res.has_errors());
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_SELECT_STATEMENT").count(), 2);
}

#[test]
fn test_go_accepted_under_mssql_dialect() {
    let res = parse(
        "select 1\nGO\nselect 2",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert_parser!(res);
}
