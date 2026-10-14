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
fn test_into_after_from_items() {
    let src = r#"do $$
begin
    select Count(*) from newOld into recCount;
    select * from ~$Функция~(fields) t into description, withClause;
    select string_agg(field_name, ',') from unnest(string_to_array(s, ',')) as tt(field_name) into field_value;
    select a from t group by a into x;
    select a from t order by a into x;
end $$"#;

    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 5);
}

#[test]
fn test_select_into_before_items() {
    let src = r#"do $$
begin
    SELECT INTO type_block, block_month "Тип", coalesce(dog."Месяц", 0)
    FROM ~Таблица~ dog WHERE dog.row_id = 1;
    select into strict x a from t;
end $$"#;

    assert_eq!(count(src, "PSQL_INTO_CLAUSE"), 2);
    assert_eq!(count(src, "STRICT_KW"), 1);
}

#[test]
fn test_broken_select_into_reports_errors() {
    for src in [
        "do $$ begin select into from t; end $$",
        "do $$ begin select into x, from t; end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
