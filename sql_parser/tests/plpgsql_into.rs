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
fn test_select_into_variables() {
    let src = r#"do $$
DECLARE
    viewMask int := 0;
    temp text;
BEGIN
    select CAST( "Значение" as int ) INTO viewMask
    from ~Таблица~
    where "Ключ" = 'ViewMask';
    SELECT a, b INTO STRICT x, rec.field FROM t;
END $$;"#;

    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 2);
    assert_eq!(count(src, "STRICT_KW"), 1);
}

#[test]
fn test_returning_into() {
    let src = r#"do $$
begin
    insert into t (a) values (1) RETURNING row_id INTO v_id;
    update t set a = 2 where a = 1 returning a, b into x, y;
    delete from t where a = 2 returning 0 into result;
end $$"#;

    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 3);
}

#[test]
fn test_select_into_new_table() {
    assert_eq!(count("select * into t2 from t", "PSQL_INTO_CLAUSE"), 1);
}

#[test]
fn test_strict_is_still_a_valid_name() {
    let res = parse("select strict from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_into_reports_errors() {
    for src in [
        "do $$ begin select a into from t; end $$",
        "do $$ begin select a into x, from t; end $$",
        "do $$ begin select a into strict; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
