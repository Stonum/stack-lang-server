#[macro_use]
mod helper;

#[test]
fn format_insert_values() {
    assert_fmt!(
        r#"--
insert into t (a, b)
values (1, 2)
"#
    );
}

#[test]
fn format_insert_select() {
    assert_fmt!(
        r#"--
insert into t (a, b)
select a, b from t2
"#
    );
}

#[test]
fn format_insert_without_columns() {
    assert_fmt!(
        r#"--
insert into t
values (1, 2)
"#
    );
}

#[test]
fn format_insert_with_returning() {
    assert_fmt!(
        r#"--
insert into t (a, b)
values (1, 2)
returning a, b
"#
    );
}

#[test]
fn format_insert_on_conflict_do_nothing() {
    assert_fmt!(
        r#"--
insert into t (a, b)
values (1, 2)
on conflict (a) do nothing
"#
    );
}

#[test]
fn format_insert_on_conflict_do_update() {
    // WHERE stays a sibling of "on conflict ... do update set", same as
    // every other clause pair in this formatter -- not indented under it.
    assert_fmt!(
        r#"--
insert into t (a, b)
values (1, 2)
on conflict (a) do update set b = 2
where a = 1
"#
    );
}

#[test]
fn format_insert_on_conflict_on_constraint() {
    assert_fmt!(
        r#"--
insert into t (a, b)
values (1, 2)
on conflict on constraint t_pkey do nothing
"#
    );
}

#[test]
fn format_insert_values_wrap_when_too_long() {
    assert_fmt!(
        r#"--
insert into t (a, b, c, d)
values (
	really_long_value_one,
	really_long_value_two,
	really_long_value_three,
	really_long_value_four
)
"#
    );
}
