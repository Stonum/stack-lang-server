#[macro_use]
mod helper;

#[test]
fn format_insert_select_with_case_cast_and_array() {
    assert_fmt!(
        r#"--
insert into t (a, b)
select case when x::int > 0 then array[1, 2, 3] else array[] end, y
from (
	select a as x, b as y from t2
) sub
returning a, b
"#
    );
}
