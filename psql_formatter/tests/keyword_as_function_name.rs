#[macro_use]
mod helper;

#[test]
fn format_right_as_function_name() {
    assert_fmt!(
        r#"--
select right(name, 4) from t
"#
    );
}

#[test]
fn format_left_as_function_name() {
    assert_fmt!(
        r#"--
select left(name, 1) from t
"#
    );
}

#[test]
fn format_right_join_still_works() {
    assert_fmt!(
        r#"--
select a
from t1
right join t2 on t1.id = t2.id
"#
    );
}

#[test]
fn format_left_join_still_works() {
    assert_fmt!(
        r#"--
select a
from t1
left join t2 on t1.id = t2.id
"#
    );
}
