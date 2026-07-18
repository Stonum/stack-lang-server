#[macro_use]
mod helper;

#[test]
fn format_union_all_between_two_branches() {
    assert_fmt!(
        r#"--
select a
from t1
	union all
select a
from t2
"#
    );
}

#[test]
fn format_union_without_quantifier() {
    assert_fmt!(
        r#"--
select a
from t1
	union
select a
from t2
"#
    );
}

#[test]
fn format_intersect_and_except_chain() {
    assert_fmt!(
        r#"--
select a
from t1
	intersect
select a
from t2
	except
select a
from t3
"#
    );
}

#[test]
fn format_union_branches_each_with_where() {
    assert_fmt!(
        r#"--
select a
from t1
where a > 1
	union all
select a
from t2
where a > 2
"#
    );
}

#[test]
fn format_union_inside_subquery_indents_with_it() {
    assert_fmt!(
        r#"--
select a
from (
	select a
	from t1
		union all
	select a
	from t2
) sub
"#
    );
}
