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
fn test_extract() {
    for src in [
        "select extract(year from age(coalesce(s.a, now()::date), s.b)) from s",
        "select EXTRACT(EPOCH FROM now() - t.c) from t",
        "select extract('day' from d) from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_EXTRACT_EXPRESSION"), "{src}");
    }
}

#[test]
fn test_position() {
    for src in [
        "select position('a' in s) from t",
        "select position(lower(x) in lower(y)) > 0 from t",
        "select position((a = b) in s) from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_POSITION_EXPRESSION"), "{src}");
    }
}

#[test]
fn test_trim() {
    for src in [
        "select trim(both ' ' from s) from t",
        "select trim(leading from s), trim(trailing '0' from s) from t",
        "select trim('x' from s) from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_TRIM_EXPRESSION"), "{src}");
    }
}

#[test]
fn test_plain_calls_stay_plain() {
    let res = parse(
        "select trim(s), trim(s, 'x'), position, extract from t",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(!tree.contains("PSQL_TRIM_EXPRESSION"));
}

#[test]
fn test_special_forms_report_errors() {
    for src in [
        "select extract(year from) from t",
        "select position('a' in) from t",
        "select trim(both from) from t",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_special_forms_rejected_under_mssql_dialect() {
    for src in [
        "select extract(year from d) from t",
        "select position('a' in s) from t",
        "select trim(both ' ' from s) from t",
    ] {
        let res = parse(src, SqlFileSource::script().with_dialect(SqlDialect::Mssql));

        assert!(res.has_errors(), "{src}");
    }
}
