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
fn test_parenthesized_right_operand() {
    let res = parse(
        "select a from t union all (select b from u order by b limit 1)",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    let set_operation = tree.find("SQL_SET_OPERATION@").unwrap();
    assert!(tree[set_operation..].contains("SQL_SUBQUERY_EXPRESSION"));
}

#[test]
fn test_parenthesized_first_operand() {
    for src in [
        "(select a from t limit 1) union all select b from u order by 1",
        "(select a from t) except (select b from u);",
        "with x as (select 1 as a) (select a from x) union select 2",
        "select x from ((select a as x from t) union (select b from u)) s",
        r#"with recursive up as ((select "Поле", 1 as sort from ~Таблица~ where "Ключ" = current_user limit 1) union all select t."Поле", up.sort + 1 from ~Таблица~ t join up on up."Поле" = t."Ключ") select * from up"#,
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_COMPOUND_SELECT_STATEMENT"), "{src}");
    }
}

#[test]
fn test_parenthesized_expressions_are_not_set_operands() {
    let res = parse(
        "select ((select 1) + 1), ((select 2)), (a + b) * c from t",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(!tree.contains("SQL_COMPOUND_SELECT_STATEMENT"));
}

#[test]
fn test_parenthesized_join_still_works() {
    let res = parse(
        "select a from (t join u on u.id = t.id) left join v on v.id = t.id",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_compound_without_second_operand_reports_error() {
    let res = parse("(select a from t) union", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_parenthesized_non_query_before_set_operator_does_not_hang() {
    // Regression test: `(1) union ...` used to start a compound statement
    // without consuming anything, tripping the parser's progress check.
    for src in [
        "(1) union select 2",
        "((a)) union select b",
        "select a from t union (1)",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
