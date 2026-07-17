#[macro_use]
mod helper;

use psql_syntax::PsqlSyntaxKind;

#[test]
fn format_select_list_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select
	really_long_column_name_one,
	really_long_column_name_two,
	really_long_column_name_three,
	really_long_column_name_four
from t
"#
    );
}

#[test]
fn format_select_list_stays_flat_when_short() {
    assert_fmt!(
        r#"--
select a, b, c
from t
"#
    );
}

#[test]
fn format_group_by_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select a
from t
group_by
	really_long_column_name_one,
	really_long_column_name_two,
	really_long_column_name_three,
	really_long_column_name_four
"#
    );
}

#[test]
fn format_order_by_wraps_when_too_long() {
    assert_fmt!(
        r#"--
select a
from t
order_by
	really_long_column_name_one desc,
	really_long_column_name_two desc,
	really_long_column_name_three desc,
	really_long_column_name_four desc
"#
    );
}

#[test]
fn format_from_items_wrap_when_too_long() {
    assert_fmt!(
        r#"--
select a
from
	really_long_table_name_one,
	really_long_table_name_two,
	really_long_table_name_three,
	really_long_table_name_four
"#
    );
}

#[test]
fn format_function_call_arguments_wrap_when_too_long() {
    // Formatted as a standalone node (not through the surrounding FROM
    // clause's own group) to isolate the argument list's own wrap decision
    // from whether the outer "from <item>" line also happens to overflow --
    // that cascading interaction (single long FROM item forcing "from"
    // itself onto its own line too) is a real, separate "hugging" nuance
    // not covered by this point.
    assert_fmt_node!(
        "select a from generate_series(really_long_argument_one, really_long_argument_two, really_long_argument_three, really_long_argument_four) g",
        PsqlSyntaxKind::PSQL_FUNCTION_BINDING,
        "generate_series(\n\treally_long_argument_one,\n\treally_long_argument_two,\n\treally_long_argument_three,\n\treally_long_argument_four\n) g"
    );
}
