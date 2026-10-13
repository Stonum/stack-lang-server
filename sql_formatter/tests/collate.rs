#[macro_use]
mod helper;

#[test]
fn format_collate_expressions_and_constraints() {
    assert_fmt!(
        r#"--
select a collate "C", (a || b) collate "und-x-icu", a::text collate "C" from t
create table t (name text collate "C" not null, code varchar(10) collate "und-x-icu")
"#
    );
}

#[test]
fn format_collate_normalizes_case_and_drops_redundant_parens() {
    assert_fmt_eq!(
        r#"--
SELECT a FROM t JOIN u ON (u.name::text COLLATE "C") = t.name
"#,
        r#"--
select a
from t
join u on u.name::text collate "C" = t.name
"#
    );
}

#[test]
fn format_long_query_with_collate_wraps() {
    assert_fmt_eq!(
        r#"--
select first_long_column_name collate "und-x-icu", second_long_column_name collate "und-x-icu" from accounts where owner_name = current_user collate "und-x-icu" order by first_long_column_name collate "C" desc
"#,
        r#"--
select first_long_column_name collate "und-x-icu", second_long_column_name collate "und-x-icu"
from accounts
where owner_name = current_user collate "und-x-icu"
order by first_long_column_name collate "C" desc
"#
    );
}
