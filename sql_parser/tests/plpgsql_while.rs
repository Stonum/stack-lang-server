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
fn test_while_loops() {
    let src = r#"do $$
declare
    i int := 0;
BEGIN
    WHILE i < 10 LOOP
        i := i + 1;
        delete from ~Таблица~ where "Ключ" = i;
    END LOOP;
    <<batches>>
    while exists (select 1 from t where a is null) loop
        update t set a = 1 where a is null;
        exit batches when i > 100;
    end loop batches;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_WHILE_STATEMENT"), 2);
    assert_eq!(count(src, "PSQL_PL_LOOP_BODY"), 2);
    assert_eq!(count(src, "PSQL_PL_LABEL"), 1);
}

#[test]
fn test_broken_while_reports_errors() {
    for src in [
        "do $$ begin while i < 10 i := 1; end loop; end $$",
        "do $$ begin while loop end loop; end $$",
        "do $$ begin while true loop end; end $$",
        "do $$ begin <<a>> while true loop end loop b; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_while_is_still_a_valid_name() {
    let res = parse("select while from t", postgres());

    assert_parser!(res);
}
