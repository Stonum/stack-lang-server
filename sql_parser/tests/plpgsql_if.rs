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
fn test_if_then() {
    let src = r#"do $$
BEGIN
    IF NOT EXISTS (select 1 from ~Таблица~ where "Ключ" = 1) THEN
        insert into ~Таблица~ ("Ключ") values (1);
    END IF;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_IF_STATEMENT"), 1);
}

#[test]
fn test_if_with_parenthesized_condition() {
    let src = "do $$ begin if( exists ( select 1 from t ) ) then delete from t; end if; end $$";

    assert_eq!(count(src, "PSQL_PL_IF_STATEMENT"), 1);
}

#[test]
fn test_if_elsif_else() {
    let src = r#"do $$
begin
    if x = 1 then
        y := 1;
    elsif x = 2 then
        y := 2;
    elseif x = 3 then
        y := 3;
    else
        y := 0;
        delete from t;
    end if;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_ELSIF_CLAUSE"), 2);
    assert_eq!(count(src, "PSQL_PL_ELSE_CLAUSE"), 1);
    assert_eq!(count(src, "PSQL_PL_ASSIGNMENT_STATEMENT"), 4);
}

#[test]
fn test_nested_if_and_empty_branch() {
    let src = r#"do $$
begin
    if a then
        if b then
        else
            begin
                delete from t;
            end;
        end if;
    end if;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_IF_STATEMENT"), 2);
    assert_eq!(count(src, "PSQL_PL_BLOCK"), 2);
}

#[test]
fn test_elsif_is_still_a_valid_name() {
    let res = parse("select elsif, elseif from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_broken_if_reports_errors_and_recovers() {
    for src in [
        "do $$ begin if x delete from t; end if; end $$",
        "do $$ begin if x then delete from t; end; end $$",
        "do $$ begin if x then delete from t; end if end $$",
        "do $$ begin if then end if; end $$",
        "do $$ begin else end $$",
        "do $$ begin if x then $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
