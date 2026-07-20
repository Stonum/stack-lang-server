#[macro_use]
mod helper;

use psql_syntax::PsqlSyntaxKind;

#[test]
fn format_select_list_wraps_when_too_long() {
    // All 4 items are "simple" (bare names), so PsqlSelectItemList's fill
    // layout packs them together rather than one per line -- see
    // format_select_list_fill_packs_simple_and_breaks_complex below for the
    // mixed simple/complex case.
    assert_fmt!(
        r#"--
select
	really_long_column_name_one, really_long_column_name_two, really_long_column_name_three, really_long_column_name_four
from t
"#
    );
}

#[test]
fn format_select_list_fill_packs_simple_and_breaks_complex() {
    // Mirrors format_function_call_arguments_fill_packs_simple_and_breaks_complex:
    // simple select items pack together, but a complex one (here, a CASE
    // expression) -- and the item right after it -- always start on their
    // own line.
    assert_fmt!(
        r#"--
select
	really_long_column_one, really_long_column_two,
	case when really_long_condition then 1 else 2 end,
	really_long_column_four, really_long_column_five
from t
"#
    );
}

#[test]
fn format_select_list_stays_flat_when_short() {
    assert_fmt!(
        r#"--
select a, b, c from t
"#
    );
}

#[test]
fn format_select_list_with_many_short_items_fill_packs() {
    // Many short columns, individually well under the line width, pack
    // multiple per line (as many as fit) once their combined width doesn't
    // fit on one line -- same fill layout as function call arguments
    // (see format_function_call_arguments_wrap_when_too_long).
    assert_fmt!(
        r#"--
select
	c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25,
	c26, c27, c28, c29, c30
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
	really_long_column_name_one, really_long_column_name_two, really_long_column_name_three, really_long_column_name_four
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
	really_long_column_name_one desc, really_long_column_name_two desc, really_long_column_name_three desc,
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
	really_long_table_name_four,
	really_long_table_name_five
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
    //
    // All 4 arguments here are "simple" (bare names), so PsqlExpressionList's
    // fill layout packs them together rather than one per line -- see
    // format_function_call_arguments_fill_packs_simple_and_breaks_complex
    // below for the mixed simple/complex case.
    assert_fmt_node!(
        "select a from generate_series(really_long_argument_one, really_long_argument_two, really_long_argument_three, really_long_argument_four) g",
        PsqlSyntaxKind::PSQL_FUNCTION_BINDING,
        "generate_series(\n\treally_long_argument_one, really_long_argument_two, really_long_argument_three, really_long_argument_four\n) g"
    );
}

#[test]
fn format_function_call_arguments_fill_packs_simple_and_breaks_complex() {
    // Mirrors mlang_formatter's own call-argument layout (see
    // pretty_format_query_call_1 in mlang_formatter/tests/new_vision.rs):
    // simple arguments (bare names/literals) pack together, but a complex
    // argument -- and the argument right after it -- always start on their
    // own line, regardless of length. `coalesce(x, y, z)` is complex here
    // because it has 3 arguments; a call with 2 or fewer simple arguments is
    // itself still considered "simple" (see [is_simple_expression]), same as
    // mlang's own depth-limited `SimpleArgument::is_simple`.
    assert_fmt_node!(
        "select a from generate_series(really_long_argument_one, really_long_argument_two, coalesce(x, y, z), really_long_argument_four, really_long_argument_five) g",
        PsqlSyntaxKind::PSQL_FUNCTION_BINDING,
        "generate_series(\n\treally_long_argument_one, really_long_argument_two,\n\tcoalesce(x, y, z),\n\treally_long_argument_four, really_long_argument_five\n) g"
    );
}
