#[macro_use]
mod helper;

#[test]
fn format_nulls_order() {
    assert_fmt!(
        r#"--
select a from t order by a nulls last, b desc nulls first
create index i on t (a nulls first, b desc nulls last)
select array_agg(a order by b nulls last), row_number() over (order by c desc nulls last) from t
"#
    );
}

#[test]
fn format_nulls_order_normalizes_case() {
    assert_fmt_eq!(
        r#"--
SELECT a FROM t ORDER BY a DESC NULLS LAST
"#,
        r#"--
select a from t order by a desc nulls last
"#
    );
}

#[test]
fn format_long_order_by_with_nulls_fills_lines() {
    assert_fmt_eq!(
        r#"--
select a from t order by first_long_column_name desc nulls last, second_long_column_name asc nulls first, third_long_column_name nulls last, fourth_long_column_name
"#,
        r#"--
select a
from t
order by
	first_long_column_name desc nulls last, second_long_column_name asc nulls first,
	third_long_column_name nulls last, fourth_long_column_name
"#
    );
}
