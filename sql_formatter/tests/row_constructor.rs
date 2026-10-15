#[macro_use]
mod helper;

#[test]
fn format_row_constructors() {
    assert_fmt_eq!(
        r#"--
SELECT ( 1 ,2 );
SELECT * FROM t WHERE (a,b) IN ((1,2),(3,4));
"#,
        r#"--
select (1, 2);
select * from t where (a, b) in ((1, 2), (3, 4));
"#
    );
}

#[test]
fn format_long_row_constructor_comparison() {
    assert_fmt_eq!(
        r#"--
select * from accounts where (accounts.first_column_name, accounts.second_column_name, accounts.third_column_name, accounts.fourth_column_name, accounts.fifth_column_name) in (select x.first_column_name, x.second_column_name, x.third_column_name, x.fourth_column_name, x.fifth_column_name from x);
"#,
        r#"--
select *
from accounts
where (
	accounts.first_column_name, accounts.second_column_name, accounts.third_column_name,
	accounts.fourth_column_name, accounts.fifth_column_name
) in (
	select x.first_column_name, x.second_column_name, x.third_column_name, x.fourth_column_name, x.fifth_column_name
	from x
);
"#
    );
}
