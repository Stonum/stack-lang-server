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
fn test_for_over_queries() {
    let src = r#"do $$
DECLARE
    rec record;
BEGIN
    FOR rec IN SELECT att.attname, t."Поле" FROM ~Таблица~ t join att on att.id = t.id WHERE t.a > 1 ORDER BY 1 LOOP
        delete from t where a = rec.attname;
    END LOOP;
    for rec in select * from t loop
    end loop;
    for rec in select a from t tbl loop
    end loop;
    for rec in (select a from t) loop
    end loop;
    for rec in with q as (select 1 as a) select a from q loop
    end loop;
    for key, val in select k, v from t group by k, v loop
    end loop;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_FOR_STATEMENT"), 6);
    assert_eq!(count(src, "PSQL_PL_LOOP_BODY"), 6);
    assert_eq!(count(src, "SQL_SUBQUERY_EXPRESSION"), 1);
}

#[test]
fn test_for_over_execute() {
    let src = r#"do $$
begin
    <<rows>>
    for rec in execute format('select * from %I', tbl) using a, b loop
        exit rows;
    end loop rows;
    for rec, val in execute query_text loop
    end loop;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_FOR_EXECUTE_SOURCE"), 2);
    assert_eq!(count(src, "PSQL_PL_USING_CLAUSE"), 1);
}

#[test]
fn test_loop_is_still_an_alias_outside_for() {
    let res = parse("select loop from t loop", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_for_reports_errors() {
    for src in [
        "do $$ begin for rec select 1 loop end loop; end $$",
        "do $$ begin for rec in loop end loop; end $$",
        "do $$ begin for rec in select 1 end loop; end $$",
        "do $$ begin for in select 1 loop end loop; end $$",
        "do $$ begin for rec in execute loop end loop; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
