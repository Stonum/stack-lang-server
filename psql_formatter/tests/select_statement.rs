#[macro_use]
mod helper;

#[test]
fn format_simple_select() {
    assert_fmt!("select a, b\nfrom t");
}

#[test]
fn format_select_star() {
    assert_fmt!("select *\nfrom t");
}

#[test]
fn format_select_with_alias() {
    assert_fmt!(
        r#"select a as "A", b "B"
from t"#
    );
}

#[test]
fn format_select_multiple_from_items() {
    assert_fmt!("select a\nfrom t1, t2");
}

#[test]
fn format_select_with_where() {
    assert_fmt!("select a\nfrom t\nwhere a > 1");
}

#[test]
fn format_select_with_group_by_having() {
    assert_fmt!("select a\nfrom t\ngroup_by a\nhaving a > 1");
}

#[test]
fn format_select_with_order_by_limit_offset() {
    assert_fmt!("select a\nfrom t\norder_by a desc\nlimit 10\noffset 5");
}

#[test]
fn format_select_with_join() {
    assert_fmt!("select a\nfrom t1\njoin t2 on t1.id = t2.id");
}

#[test]
fn format_select_with_multiple_joins() {
    assert_fmt!("select a\nfrom t1\nleft outer join t2 on t1.id = t2.id\njoin t3 on t2.id = t3.id");
}

#[test]
fn format_select_with_function_from() {
    assert_fmt!("select a\nfrom generate_series(1, 10) g");
}

#[test]
fn format_select_semicolon() {
    assert_fmt!("select a\nfrom t;");
}

#[test]
fn format_select_with_cross_join() {
    assert_fmt!("select a\nfrom t1\ncross join t2");
}

#[test]
fn format_select_with_complex_condition_falls_back_to_verbatim() {
    // The condition's own expression kind (logical/binary chain) isn't
    // formatted yet -- Point 5/6's job -- but it must still round-trip
    // untouched through the now-real where/join clause structure.
    assert_fmt!("select a\nfrom t\nwhere a > 1 and b < 2 or c = 3");
}

#[test]
fn format_select_full_statement() {
    assert_fmt!(
        "select a, b\nfrom t1\njoin t2 on t1.id = t2.id\nwhere a > 1\ngroup_by a\nhaving a > 1\norder_by a desc\nlimit 10\noffset 5"
    );
}
