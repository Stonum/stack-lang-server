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
fn test_simple_case_statement() {
    let src = r#"do $$
BEGIN
    CASE x
        WHEN 1, 2 THEN
            msg := 'one or two';
        WHEN 3 THEN
            delete from ~Таблица~ where "Ключ" = x;
            msg := 'three';
        ELSE
            msg := 'other';
    END CASE;
END $$;"#;

    assert_eq!(count(src, "PSQL_PL_CASE_STATEMENT"), 1);
    assert_eq!(count(src, "PSQL_PL_CASE_WHEN_CLAUSE"), 2);
    assert_eq!(count(src, "PSQL_PL_ELSE_CLAUSE"), 1);
}

#[test]
fn test_searched_case_statement_with_case_expressions_inside() {
    let src = r#"do $$
begin
    case
        when x between 0 and 10 then
            y := case when x > 5 then 'b' else 'a' end;
        when x > 10 then
            if y then
                exit;
            end if;
    end case;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_CASE_STATEMENT"), 1);
    assert_eq!(count(src, "SQL_CASE_EXPRESSION"), 1);
    assert_eq!(count(src, "PSQL_PL_CASE_WHEN_CLAUSE"), 2);
}

#[test]
fn test_broken_case_statements_report_errors() {
    for src in [
        "do $$ begin case x end case; end $$",
        "do $$ begin case when then y := 1; end case; end $$",
        "do $$ begin case when x y := 1; end case; end $$",
        "do $$ begin case when x then y := 1; end; end $$",
        "do $$ begin case when x then y := 1; end case end $$",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
