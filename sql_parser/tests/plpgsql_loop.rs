#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn count(src: &str, kind: &str) -> usize {
    let res = parse(src, postgres());
    assert_parser!(res);
    format!("{:#?}", res.syntax())
        .matches(&format!("{kind}@"))
        .count()
}

#[test]
fn test_loop_with_exit_and_continue() {
    let src = r#"do $$
declare
    i int := 0;
BEGIN
    LOOP
        i := i + 1;
        CONTINUE WHEN i < 3;
        EXIT WHEN i > 10;
        if i = 5 then
            exit;
        end if;
        delete from ~Таблица~ where "Ключ" = i;
    END LOOP;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_LOOP_STATEMENT"), 1);
    assert_eq!(count(src, "PSQL_PL_EXIT_STATEMENT"), 3);
    assert_eq!(count(src, "PSQL_PL_EXIT_WHEN_CLAUSE"), 2);
}

#[test]
fn test_labels() {
    let src = r#"do $$
<<outer_block>>
declare
    x int;
begin
    <<outer_loop>>
    loop
        <<inner_loop>>
        loop
            exit outer_loop when x > 1;
            continue inner_loop;
        end loop inner_loop;
    end loop outer_loop;
    <<nested>>
    begin
        exit nested;
    end nested;
end outer_block $$"#;

    assert_eq!(count(src, "PSQL_PL_LABEL"), 4);
    assert_eq!(count(src, "PSQL_PL_LOOP_STATEMENT"), 2);
    assert_eq!(count(src, "PSQL_PL_BLOCK"), 2);
}

#[test]
fn test_loop_keywords_are_still_valid_names() {
    let res = parse("select loop, exit from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_loops_report_errors() {
    for src in [
        "do $$ begin loop delete from t; end; end $$",
        "do $$ begin loop delete from t; end loop end $$",
        "do $$ begin exit when; end $$",
        "do $$ begin <<a loop end loop; end $$",
        "do $$ begin <<a>> delete from t; end $$",
        "do $$ begin loop $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_mismatched_end_labels_report_errors() {
    for src in [
        "do $$ begin end extra $$",
        "do $$ <<a>> begin end b $$",
        "do $$ begin loop end loop extra; end $$",
        "do $$ begin <<a>> loop end loop b; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }

    let res = parse(
        "do $$ <<Main>> begin <<a>> loop end loop A; end main $$",
        postgres(),
    );
    assert_parser!(res);
}
