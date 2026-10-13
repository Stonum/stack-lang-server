#[macro_use]
mod helper;

#[test]
fn format_delete_simple() {
    assert_fmt!(
        r#"--
delete from t
"#
    );
}

#[test]
fn format_delete_with_where() {
    assert_fmt!(
        r#"--
delete from t where a = 1
"#
    );
}

#[test]
fn format_delete_with_using() {
    assert_fmt!(
        r#"--
delete from t using t2 where t.id = t2.id
"#
    );
}

#[test]
fn format_delete_with_returning() {
    assert_fmt!(
        r#"--
delete from t where a = 1 returning a
"#
    );
}

#[test]
fn format_delete_using_multiple_tables_packs_when_it_fits() {
    assert_fmt!(
        r#"--
delete from t using a, b, c
"#
    );
}

#[test]
fn format_delete_using_multiple_tables_wraps_when_too_long() {
    assert_fmt!(
        r#"--
delete from t
using
	really_long_table_name_one, really_long_table_name_two, really_long_table_name_three,
	really_long_table_name_four, really_long_table_name_five
"#
    );
}

#[test]
fn format_delete_stays_on_one_line_when_it_fits_and_wraps_otherwise() {
    assert_fmt_eq!(
        r#"--
DELETE FROM t
WHERE a = 1;
delete from accounts_history using accounts where accounts_history.account = accounts.id and accounts.deleted is not null returning accounts_history.id;
"#,
        r#"--
delete from t where a = 1;
delete from accounts_history
using accounts
where accounts_history.account = accounts.id and accounts.deleted is not null
returning accounts_history.id;
"#
    );
}
