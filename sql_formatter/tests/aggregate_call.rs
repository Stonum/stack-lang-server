#[macro_use]
mod helper;

#[test]
fn format_aggregate_quantifiers_and_order_by() {
    assert_fmt!(
        r#"--
select count(distinct a), sum(all x), count(*) from t
select string_agg(distinct name, ', ' order by name desc) from t
"#
    );
}

#[test]
fn format_aggregate_normalizes_keyword_case() {
    assert_fmt_eq!(
        r#"--
select array_agg(DISTINCT a ORDER BY b DESC) from t
"#,
        r#"--
select array_agg(distinct a order by b desc) from t
"#
    );
}

#[test]
fn format_long_aggregate_moves_order_by_to_its_own_line() {
    assert_fmt_eq!(
        r#"--
select string_agg(owners.full_name || ' (' || owners.code || ')', ', ' order by owners.full_name, owners.code desc) as names from owners
"#,
        r#"--
select string_agg(
	owners.full_name || ' (' || owners.code || ')',
	', '
	order by owners.full_name, owners.code desc
) as names
from owners
"#
    );
}

#[test]
fn format_aggregate_many_short_arguments_fill_lines() {
    assert_fmt_eq!(
        r#"--
select json_agg(distinct first_long_column_name, second_long_column_name, third_long_column_name order by first_long_column_name desc, second_long_column_name) from t
"#,
        r#"--
select json_agg(
	distinct first_long_column_name, second_long_column_name, third_long_column_name
	order by first_long_column_name desc, second_long_column_name
)
from t
"#
    );
}
