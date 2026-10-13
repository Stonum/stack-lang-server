#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_do_statement_forms() {
    for src in [
        "do $$ begin perform 1; end $$",
        "DO LANGUAGE plpgsql $$ BEGIN END $$;",
        "do $body$ begin null; end $body$ language plpgsql",
        "do 'begin null; end'",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_DO_STATEMENT"), "{src}");
    }
}

#[test]
fn test_do_in_mlang_batch_and_on_conflict_do() {
    let res = parse(
        "do language plpgsql $$ begin drop table if exists ~Таблица~; end $$\ninsert into t (a) values (1) on conflict (a) do nothing",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_DO_STATEMENT"));
    assert!(tree.contains("SQL_INSERT_STATEMENT"));
}

#[test]
fn test_do_without_body_reports_error() {
    for src in ["do", "do language plpgsql;"] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_do_rejected_under_mssql_dialect() {
    let res = parse(
        "do $$ begin end $$",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}
