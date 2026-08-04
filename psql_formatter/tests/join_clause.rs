#[macro_use]
mod helper;

#[test]
fn format_join_two_conditions_stays_flat_even_when_long() {
    assert_fmt!(
        r#"--
select a
from t1
join t2 on really_long_column_name_one = t2.id and really_long_column_name_two = t2.x
"#
    );
}

#[test]
fn format_join_three_conditions_wraps() {
    assert_fmt!(
        r#"--
select a
from t1
join t2 on t1.id = t2.id
	and t1.x = t2.x
	and t1.y = t2.y
"#
    );
}

#[test]
fn format_multiple_joins_each_wrap_independently() {
    // First join has 3 conditions (wraps), second has just 1 (doesn't) --
    // each join's own condition wrapping must not affect the others', and
    // every join line stays at the same indent level (siblings of "from").
    assert_fmt!(
        r#"--
select a
from t1
join t2 on t1.id = t2.id
	and t1.x = t2.x
	and t1.y = t2.y
join t3 on t2.id = t3.id
"#
    );
}

#[test]
fn format_cross_join_alongside_wrapped_join() {
    assert_fmt!(
        r#"--
select a
from t1
join t2 on t1.id = t2.id
	and t1.x = t2.x
	and t1.y = t2.y
cross join t3
"#
    );
}

#[test]
fn format_join_or_condition_wraps() {
    assert_fmt!(
        r#"--
select a
from t1
join t2 on t1.id = t2.id
	or t1.id = t2.alt_id
	or t1.id = t2.legacy_id
"#
    );
}

#[test]
fn format_join_lateral_subquery() {
    assert_fmt!(
        r#"--
select a
from t1
join lateral (select max(b) from t2 where t2.t1_id = t1.id) x on true
"#
    );
}

#[test]
fn format_left_join_lateral_function() {
    assert_fmt!(
        r#"--
select a
from t1
left join lateral unnest(t1.arr) x on true
"#
    );
}

#[test]
fn format_from_with_lateral_subquery() {
    assert_fmt!(
        r#"--
select a from t1, lateral (select max(b) from t2 where t2.t1_id = t1.id) x
"#
    );
}

#[test]
fn format_lateral_normalizes_case() {
    assert_fmt_eq!(
        r#"--
select a from t1, LATERAL generate_series(1, t1.n) g;
"#,
        r#"--
select a from t1, lateral generate_series(1, t1.n) g;
"#
    );
}
