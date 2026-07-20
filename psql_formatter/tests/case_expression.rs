#[macro_use]
mod helper;

#[test]
fn format_short_case_stays_flat() {
    // A short CASE collapses onto one line when it fits, same as a simple
    // SELECT -- it only wraps once it doesn't (see the *_wraps_when_too_long
    // tests below).
    assert_fmt!(
        r#"--
select case when a then 1 end from t
"#
    );
}

#[test]
fn format_searched_case_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select case
	when really_long_condition_one then really_long_result_one
	when really_long_condition_two then really_long_result_two
	else really_long_default_result
end
from t
"#
    );
}

#[test]
fn format_simple_case_with_expression_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select case really_long_case_subject_expression
	when really_long_match_value_one then really_long_result_one
	when really_long_match_value_two then really_long_result_two
end
from t
"#
    );
}

#[test]
fn format_case_without_else_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select case
	when really_long_condition_that_forces_a_wrap_all_by_itself_here_even_at_a_much_wider_line_width_than_before then 'x'
end
from t
"#
    );
}

#[test]
fn format_select_with_other_items_before_and_after_case() {
    // The CASE itself is too long to stay flat, which means the *whole*
    // flattened "a, case ... end, c" text also doesn't fit -- so the outer
    // select-item list independently reaches the same "must expand"
    // conclusion and puts every item on its own line too, not just the
    // CASE's own internal branches.
    assert_fmt!(
        r#"--
select
	a,
	case
		when really_long_condition_one then really_long_result_one
		when really_long_condition_two then really_long_result_two
	end,
	c
from t
"#
    );
}

#[test]
fn format_select_case_with_alias_and_other_short_items() {
    assert_fmt!(
        r#"--
select
	a,
	case
		when really_long_condition_one then really_long_result_one
		when really_long_condition_two then really_long_result_two
	end as label,
	c
from t
"#
    );
}

#[test]
fn format_select_multiple_short_items_including_short_case() {
    assert_fmt!(
        r#"--
select a, case when b then 1 end, c from t
"#
    );
}
