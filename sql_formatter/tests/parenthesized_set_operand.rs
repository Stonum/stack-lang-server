#[macro_use]
mod helper;

#[test]
fn format_parenthesized_set_operands() {
    assert_fmt!(
        r#"--
(select a from t limit 1)
	union all
select b from u
order by 1
"#
    );
}

#[test]
fn format_parenthesized_right_operand_normalizes_case() {
    assert_fmt_eq!(
        r#"--
SELECT a FROM t UNION ALL (SELECT b FROM u ORDER BY b LIMIT 1)
"#,
        r#"--
select a
from t
	union all
(select b from u order by b limit 1)
"#
    );
}

#[test]
fn format_recursive_cte_with_parenthesized_anchor_wraps() {
    assert_fmt_eq!(
        r#"--
with recursive up as ((select a, 1 as sort from t where b = current_user limit 1) union all select t.a, up.sort + 1 from t join up on up.a = t.p) select * from up
"#,
        r#"--
with recursive up as (
	(select a, 1 as sort from t where b = current_user limit 1)
		union all
	select t.a, up.sort + 1
	from t
	join up on up.a = t.p
)
select * from up
"#
    );
}

#[test]
fn format_parenthesized_union_in_from_wraps() {
    assert_fmt_eq!(
        r#"--
select x from ((select a as x from t) union (select b from u)) s
"#,
        r#"--
select x
from (
	(select a as x from t)
		union
	(select b from u)
) s
"#
    );
}
