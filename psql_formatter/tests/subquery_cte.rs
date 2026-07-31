#[macro_use]
mod helper;

#[test]
fn format_cte_body_stays_flat_when_it_fits() {
    assert_fmt!(
        r#"--
with x as (select a from t)
select a from x
"#
    );
}

#[test]
fn format_cte_body_wraps_when_too_long() {
    assert_fmt!(
        r#"--
with really_long_cte_name as (
	select really_long_column_name_number_one, really_long_column_name_number_two, really_long_column_name_number_three
	from t
)
select a from really_long_cte_name
"#
    );
}

#[test]
fn format_multiple_ctes() {
    // Consistent with every other wrapped list (Point 4): trailing comma
    // ends the previous item's line, the next item always starts a fresh
    // line -- no "hugging" the comma onto the closing `)`.
    assert_fmt!(
        r#"--
with really_long_cte_name_one as (
	select really_long_column_name_number_one, really_long_column_name_number_two, really_long_column_name_number_three
	from t1
),
really_long_cte_name_two as (
	select really_long_column_name_number_four, really_long_column_name_number_five, really_long_column_name_number_six
	from t2
)
select a from x
"#
    );
}

#[test]
fn format_subquery_in_from_stays_flat_when_it_fits() {
    assert_fmt!(
        r#"--
select a from (select a from t) sub
"#
    );
}

#[test]
fn format_subquery_in_from_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select a
from (
	select really_long_column_name_number_one, really_long_column_name_number_two, really_long_column_name_number_three
	from t
) sub
"#
    );
}

#[test]
fn format_subquery_in_in_expression_stays_flat_when_it_fits() {
    // Every subquery position (`PsqlSubqueryExpression`, a CTE body, a
    // `from`-subquery) shares the same "wrap only if it doesn't fit"
    // behavior -- none of them force a break unconditionally.
    assert_fmt!(
        r#"--
select a from t where a in (select b from t2)
"#
    );
}

#[test]
fn format_nested_subquery_stays_flat_when_it_fits() {
    assert_fmt!(
        r#"--
select a from (select a from (select a from t) inner_sub) outer_sub
"#
    );
}

#[test]
fn format_nested_subquery_wraps_each_level_when_too_long() {
    assert_fmt!(
        r#"--
select a
from (
	select a
	from (
		select really_long_column_name_number_one, really_long_column_name_number_two, really_long_column_name_number_three
		from t
	) inner_sub
) outer_sub
"#
    );
}
