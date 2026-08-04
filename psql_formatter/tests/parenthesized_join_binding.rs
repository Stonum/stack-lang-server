#[macro_use]
mod helper;

#[test]
fn format_parenthesized_join_as_outer_join_source() {
    assert_fmt!(
        r#"--
select a
from (
	t1
	join t2 on t1.id = t2.id
)
left join t3 on t2.id = t3.id
"#
    );
}

#[test]
fn format_parenthesized_join_with_alias() {
    assert_fmt!(
        r#"--
select a
from (
	t1
	join t2 on t1.id = t2.id
) x
"#
    );
}

#[test]
fn format_nested_parenthesized_join() {
    assert_fmt!(
        r#"--
select a
from (
	(
		t1
		join t2 on t1.id = t2.id
	)
	join t3 on t2.id = t3.id
)
"#
    );
}

#[test]
fn format_subquery_unaffected_by_parenthesized_join_support() {
    assert_fmt!(
        r#"--
select a from (select b from t) x
"#
    );
}

#[test]
fn format_left_join_of_parenthesized_tilde_join_with_and_condition_and() {
    // Real-world-shaped, see the parser test of the same name.
    assert_fmt!(
        r#"--
select *
from t0
left join (
	~Object~ o
	join ~Object Params~ p on o.id = p.id
) on o.acc = t0.acc
"#
    );
}

#[test]
fn format_parenthesized_join_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
select a from  (  t1   JOIN t2 ON t1.id=t2.id  )   x;
"#,
        r#"--
select a
from (
	t1
	join t2 on t1.id = t2.id
) x;
"#
    );
}
