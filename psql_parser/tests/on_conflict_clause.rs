#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_on_conflict_do_nothing_bare() {
    let res = parse(
        "insert into t values (1) on conflict do nothing",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_target_do_nothing() {
    let res = parse(
        "insert into t (id, a) values (1, 2) on conflict (id) do nothing",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_multiple_target_columns() {
    let res = parse(
        "insert into t (a, b) values (1, 2) on conflict (a, b) do nothing",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_do_update() {
    let res = parse(
        "insert into t (id, a) values (1, 2) on conflict (id) do update set a = 2",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_do_update_with_where() {
    let res = parse(
        "insert into t (id, a) values (1, 2) on conflict (id) do update set a = 2 where t.a < 2",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_do_update_multiple_set_items() {
    let res = parse(
        "insert into t (id, a, b) values (1, 2, 3) on conflict (id) do update set a = 2, b = 3",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_on_constraint() {
    let res = parse(
        "insert into t values (1) on conflict on constraint t_pkey do nothing",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_with_returning() {
    let res = parse(
        "insert into t (id, a) values (1, 2) on conflict (id) do update set a = 2 returning id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_do_nothing_with_returning() {
    let res = parse(
        "insert into t values (1) on conflict do nothing returning id",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_trailing_semicolon() {
    let res = parse(
        "insert into t values (1) on conflict do nothing;",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_in_insert_select() {
    let res = parse(
        "insert into t (id, a) select id, a from u on conflict (id) do update set a = u.a",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_on_conflict_in_data_modifying_cte() {
    let res = parse(
        "with moved as (insert into t (id) values (1) on conflict (id) do nothing returning id) select id from moved",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}
