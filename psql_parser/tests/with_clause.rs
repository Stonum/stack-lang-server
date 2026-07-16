#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_with_select() {
    let res = parse(
        "with cte as (select a from t) select a from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_recursive() {
    let res = parse(
        "with recursive cte as (select a from t) select a from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_column_list() {
    let res = parse(
        "with cte (a, b) as (select a, b from t) select a from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_multiple_ctes() {
    let res = parse(
        "with cte1 as (select a from t), cte2 as (select b from u) select a from cte1 join cte2 on cte1.a = cte2.b",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_insert() {
    let res = parse(
        "with cte as (select a from t) insert into u select a from cte",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_update() {
    let res = parse(
        "with cte as (select id from t) update u set a = 1 where u.id in (select id from cte)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_delete() {
    let res = parse(
        "with cte as (select id from t) delete from u where u.id in (select id from cte)",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_with_trailing_semicolon() {
    let res = parse(
        "with cte as (select a from t) select a from cte;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_with_statements() {
    let res = parse(
        "with cte as (select a from t) select a from cte; with cte2 as (select b from u) select b from cte2;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_data_modifying_cte_insert() {
    let res = parse(
        "with moved as (insert into archive select * from t returning id) select id from moved",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_data_modifying_cte_update() {
    let res = parse(
        "with updated as (update t set a = 1 where id = 1 returning id) select id from updated",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_data_modifying_cte_delete() {
    let res = parse(
        "with deleted as (delete from t where id = 1 returning id) select id from deleted",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
