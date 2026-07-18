#[macro_use]
mod helper;

#[test]
fn format_cte_body_indents_one_level() {
    assert_fmt!(
        r#"--
with x as (
	select a
	from t
)
select a
from x
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
with x as (
	select a
	from t1
),
y as (
	select b
	from t2
)
select a
from x
"#
    );
}

#[test]
fn format_subquery_in_from_indents_one_level() {
    assert_fmt!(
        r#"--
select a
from (
	select a
	from t
) sub
"#
    );
}

#[test]
fn format_subquery_in_in_expression_indents_one_level() {
    assert_fmt!(
        r#"--
select a
from t
where a in (
	select b
	from t2
)
"#
    );
}

#[test]
fn format_nested_subquery_indents_each_level() {
    assert_fmt!(
        r#"--
select a
from (
	select a
	from (
		select a
		from t
	) inner_sub
) outer_sub
"#
    );
}
