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
fn test_do_body_is_parsed_as_plpgsql() {
    for src in [
        "do $$ begin drop table if exists t; end $$",
        "do language plpgsql $$ begin update ~Таблица~ set \"Поле\" = 1; end $$;",
        "do $body$ begin end $body$ language 'plpgsql'",
        "do $$ begin delete from t; begin truncate u; end; end; $$",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_PL_BODY"), "{src}");
        assert!(!tree.contains("SQL_BEGIN_STATEMENT"), "{src}");
    }
}

#[test]
fn test_other_languages_and_quoted_bodies_stay_strings() {
    for src in [
        "do $py$ print(1) $py$ language plpython3u",
        "do language sql $$ select 1 $$",
        "do 'begin null; end'",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(!tree.contains("PSQL_PL_BODY"), "{src}");
    }
}

#[test]
fn test_nested_dollar_strings_with_another_tag_stay_strings() {
    let res = parse(
        "do $body$ begin update t set a = $x$ text $x$; end $body$",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_statements_after_a_do_block_still_parse() {
    let res = parse(
        "do $$ begin delete from t; end $$;\nselect a from t;",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_SELECT_STATEMENT"));
}

#[test]
fn test_invalid_plpgsql_reports_errors_inside_the_body() {
    for src in [
        "do $$ begin delete from t; $$",
        "do $$ delete from t; $$",
        "do $$ begin garbage here; delete from t; end $$",
        "do $$ begin end extra $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
