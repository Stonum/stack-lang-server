#[macro_use]
mod helper;

#[test]
fn format_simple_select() {
    assert_fmt!(
        r#"--
select a, b from t
"#
    );
}

#[test]
fn format_select_star() {
    assert_fmt!(
        r#"--
select * from t
"#
    );
}

#[test]
fn format_select_table_qualified_star() {
    assert_fmt!(
        r#"--
select t.* from t
"#
    );
}

#[test]
fn format_select_table_qualified_star_with_other_columns() {
    assert_fmt!(
        r#"--
select t.*, u.a
from t
join u on t.id = u.id
"#
    );
}

#[test]
fn format_select_schema_qualified_star() {
    assert_fmt!(
        r#"--
select s.t.* from s.t
"#
    );
}

#[test]
fn format_select_with_alias() {
    assert_fmt!(
        r#"--
select a as "A", b "B" from t
"#
    );
}

#[test]
fn format_select_multiple_from_items() {
    assert_fmt!(
        r#"--
select a from t1, t2
"#
    );
}

#[test]
fn format_select_with_where() {
    assert_fmt!(
        r#"--
select a from t where a > 1
"#
    );
}

#[test]
fn format_select_with_group_by_having() {
    assert_fmt!(
        r#"--
select a from t group by a having a > 1
"#
    );
}

#[test]
fn format_select_with_order_by_limit_offset() {
    assert_fmt!(
        r#"--
select a from t order by a desc limit 10 offset 5
"#
    );
}

#[test]
fn format_select_with_join() {
    assert_fmt!(
        r#"--
select a
from t1
join t2 on t1.id = t2.id
"#
    );
}

#[test]
fn format_select_with_multiple_joins() {
    assert_fmt!(
        r#"--
select a
from t1
left outer join t2 on t1.id = t2.id
join t3 on t2.id = t3.id
"#
    );
}

#[test]
fn format_select_with_function_from() {
    assert_fmt!(
        r#"--
select a from generate_series(1, 10) g
"#
    );
}

#[test]
fn format_select_with_tilde_table_valued_function_from() {
    assert_fmt!(
        r#"--
select a from ~SomeFunc~(:1, :2, :3, :4)
"#
    );
}

#[test]
fn format_select_with_tilde_table_valued_function_alias() {
    assert_fmt!(
        r#"--
select a from ~AnotherFunc~(:root, 0) c
"#
    );
}

#[test]
fn format_select_with_tilde_name_without_parens_stays_a_table_binding() {
    assert_fmt!(
        r#"--
select a from ~Договор~ d
"#
    );
}

#[test]
fn format_select_semicolon() {
    assert_fmt!(
        r#"--
select a from t;
"#
    );
}

#[test]
fn format_select_with_cross_join() {
    assert_fmt!(
        r#"--
select a
from t1
cross join t2
"#
    );
}

#[test]
fn format_select_with_complex_condition_normalizes_mixed_and_or() {
    // `and` binds tighter than `or`, so `a > 1 and b < 2 or c = 3` parses
    // as `(a > 1 and b < 2) or c = 3` without needing parens to preserve
    // that grouping -- but the formatter adds them anyway for readability
    // whenever `and`/`or` mix without an explicit grouping (see
    // `NeedsParentheses` for `PsqlLogicalExpression`).
    assert_fmt_eq!(
        r#"--
select a from t where a > 1 and b < 2 or c = 3
"#,
        r#"--
select a from t where (a > 1 and b < 2) or c = 3
"#
    );
}

#[test]
fn format_select_distinct() {
    assert_fmt!(
        r#"--
select distinct a, b from t
"#
    );
}

#[test]
fn format_select_all() {
    assert_fmt!(
        r#"--
select all a, b from t
"#
    );
}

#[test]
fn format_select_distinct_on() {
    assert_fmt!(
        r#"--
select distinct on (a, b) a, b, c from t
"#
    );
}

#[test]
fn format_select_distinct_star() {
    assert_fmt!(
        r#"--
select distinct * from t
"#
    );
}

#[test]
fn format_select_distinct_on_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
SELECT   DISTINCT   ON  (  a , b )   a,b,c   FROM   t;
"#,
        r#"--
select distinct on (a, b) a, b, c from t;
"#
    );
}

#[test]
fn format_select_distinct_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select distinct
	really_long_column_name_one, really_long_column_name_two, really_long_column_name_three,
	really_long_column_name_four, really_long_column_name_five, really_long_column_name_six
from t
"#
    );
}

#[test]
fn format_select_normalizes_order_by_group_by_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select a, b from t  ORDER   BY  a desc;
"#,
        r#"--
select a, b from t order by a desc;
"#
    );
    assert_fmt_eq!(
        r#"--
select a from t  GROUP by  a having a > 1;
"#,
        r#"--
select a from t group by a having a > 1;
"#
    );
}

#[test]
fn format_select_full_statement() {
    assert_fmt!(
        r#"--
select a, b
from t1
join t2 on t1.id = t2.id
where a > 1
group by a
having a > 1
order by a desc
limit 10
offset 5
"#
    );
}
