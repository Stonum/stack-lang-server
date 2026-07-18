#[macro_use]
mod helper;

#[test]
fn format_update_set() {
    assert_fmt!(
        r#"--
update t
set a = 1
"#
    );
}

#[test]
fn format_update_set_multiple() {
    assert_fmt!(
        r#"--
update t
set a = 1, b = 2
"#
    );
}

#[test]
fn format_update_with_where() {
    assert_fmt!(
        r#"--
update t
set a = 1
where b = 2
"#
    );
}

#[test]
fn format_update_with_returning() {
    assert_fmt!(
        r#"--
update t
set a = 1
where b = 2
returning a, b
"#
    );
}

#[test]
fn format_update_set_wraps_when_too_long() {
    assert_fmt!(
        r#"--
update t
set
	really_long_column_name_one = 1,
	really_long_column_name_two = 2,
	really_long_column_name_three = 3,
	really_long_column_name_four = 4
"#
    );
}

#[test]
fn format_update_with_alias() {
    assert_fmt!(
        r#"--
update t as tt
set a = 1
"#
    );
}
