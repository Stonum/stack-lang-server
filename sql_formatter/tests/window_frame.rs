#[macro_use]
mod helper;

#[test]
fn format_window_frames() {
    assert_fmt!(
        r#"--
select sum(a) over (order by b rows between unbounded preceding and current row) from t
select avg(a) over (order by b range 3 preceding) from t
select count(*) over (rows between current row and unbounded following) from t
"#
    );
}

#[test]
fn format_window_frame_normalizes_case() {
    assert_fmt_eq!(
        r#"--
SELECT sum(a) OVER (ORDER BY b ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING) FROM t
"#,
        r#"--
select sum(a) over (order by b rows between unbounded preceding and 1 preceding) from t
"#
    );
}

#[test]
fn format_long_window_puts_each_clause_on_its_own_line() {
    assert_fmt_eq!(
        r#"--
select sum(payments.amount) over (partition by payments.owner, payments.kind order by payments.created desc, payments.id rows between unbounded preceding and current row) as running_total from payments
"#,
        r#"--
select sum(payments.amount) over (
	partition by payments.owner, payments.kind
	order by payments.created desc, payments.id
	rows between unbounded preceding and current row
) as running_total
from payments
"#
    );
}
