#[macro_use]
mod helper;

#[test]
fn format_standalone_values_statement() {
    assert_fmt!(
        r#"--
values (1, 2), (3, 4)
"#
    );
}

#[test]
fn format_with_prefixed_standalone_values() {
    assert_fmt!(
        r#"--
with x as (select 1)
values (1, 2)
"#
    );
}

#[test]
fn format_values_as_cte_body() {
    assert_fmt!(
        r#"--
with cte as (values (1, 2), (3, 4))
select * from cte
"#
    );
}

#[test]
fn format_values_as_cte_body_with_column_list() {
    assert_fmt!(
        r#"--
with cte(a, b) as (values (1, 2), (3, 4))
select * from cte
"#
    );
}

#[test]
fn format_values_as_derived_table() {
    assert_fmt!(
        r#"--
select * from (values (1, 2), (3, 4)) as v(a, b)
"#
    );
}

#[test]
fn format_values_as_scalar_subquery() {
    assert_fmt!(
        r#"--
select (values (1))
"#
    );
}

#[test]
fn format_insert_values_multiple_rows() {
    assert_fmt!(
        r#"--
insert into t
values (1, 2), (3, 4)
"#
    );
}

#[test]
fn format_insert_values_multiple_rows_with_columns() {
    assert_fmt!(
        r#"--
insert into t (a, b)
values (1, 2), (3, 4)
"#
    );
}

#[test]
fn format_values_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
VALUES   (1,2)  , (3,   4);
"#,
        r#"--
values (1, 2), (3, 4);
"#
    );
}
