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
	really_long_column_name_one = 1, really_long_column_name_two = 2,
	really_long_column_name_three = 3, really_long_column_name_four = 4
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

#[test]
fn format_update_with_from_clause() {
    assert_fmt!(
        r#"--
update t
set a = 1
from u
where t.id = u.id
"#
    );
}

#[test]
fn format_update_with_from_clause_multiple_items() {
    assert_fmt!(
        r#"--
update t
set a = 1, b = 2
from u, v
where t.id = u.id
"#
    );
}

#[test]
fn format_update_from_clause_wraps_when_too_long() {
    assert_fmt!(
        r#"--
update t
set a = 1
from
	really_long_table_name_one, really_long_table_name_two, really_long_table_name_three,
	really_long_table_name_four, really_long_table_name_five
where t.id = 1
"#
    );
}

#[test]
fn format_update_with_from_clause_and_returning() {
    assert_fmt!(
        r#"--
update t
set a = 1
from u
where t.id = u.id
returning t.id
"#
    );
}

#[test]
fn format_update_from_clause_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
UPDATE   t   SET   a=1,b=2   FROM   u,v   WHERE   t.id=u.id;
"#,
        r#"--
update t
set a = 1, b = 2
from u, v
where t.id = u.id;
"#
    );
}
