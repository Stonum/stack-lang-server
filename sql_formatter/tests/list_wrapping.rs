#[macro_use]
mod helper;

use sql_syntax::SqlSyntaxKind;

#[test]
fn format_select_list_packing_is_idempotent_when_a_simple_call_needs_reformatting() {
    // Regression test: `balanced_fill_breaks` used each item's raw *source*
    // text length instead of its *formatted* length. `coalesce(a,-1)` gains
    // a byte once formatted (`coalesce(a, -1)`), enough to flip a packing
    // decision on the next pass. The exact query text matters -- it's the
    // width arithmetic that hits the boundary, not the content itself.
    use biome_formatter::{IndentStyle, IndentWidth};
    use sql_formatter::{SqlFormatOptions, format_node};
    use sql_parser::parse;
    use sql_syntax::{SqlDialect, SqlFileSource};

    let src = "select (case when ok.НомерУслуги is not null then ro.НомерУслуги else ro.Услуга end) Услуга\n                              , ro.УКДоговор\n                              , coalesce(ukd.\"Организация-УКДоговор\",-1) Поставщик\n                              , ro.Аналитика1\n                              , round(sum(Сумма)::numeric, 2) as Сумма\n                         from ~РаспределениеОплатыФискализация~ ro\n                         left join #ok_temp_serv ok on ok.НомерУслуги = ro.НомерУслуги\n                         left join ~УК Договоры~ ukd on ukd.row_id = ro.УКДоговор\n                         where ro.Тип = 2 /* 2 - зачет авансов текущих платежей */\n                            and ro.Месяц = :mes\n                            and ro.Чек is null\n                         group by (case when ok.НомерУслуги is not null then ro.НомерУслуги else ro.Услуга end)\n                            , ro.УКДоговор, coalesce(ukd.\"Организация-УКДоговор\",-1), ro.Аналитика1\n                         order by Услуга";

    let syntax = SqlFileSource::query()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true);
    let tree = parse(src, syntax);
    assert!(!tree.has_errors(), "parse errors: {:?}", tree.diagnostics());

    let options = SqlFormatOptions::new(syntax)
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(IndentWidth::from(3));

    let pass1 = format_node(options.clone(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();

    let tree2 = parse(&pass1, syntax);
    let pass2 = format_node(options, &tree2.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();

    assert_eq!(
        pass1, pass2,
        "formatting is not idempotent:\nfirst pass:\n======\n{pass1}\n======\nsecond pass:\n======\n{pass2}\n======\n"
    );
}

#[test]
fn format_select_list_wraps_when_too_long() {
    // All 4 items are "simple" (bare names), so SqlSelectItemList's fill
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
    // Once packing needs more than one line, the two lines are balanced to
    // be roughly equal width instead of cramming the first line to the
    // limit and leaving a short, ragged remainder -- see
    // `balanced_fill_breaks` in `sql_formatter/src/utils.rs`.
    assert_fmt!(
        r#"--
select
	c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16,
	c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30
from t
"#
    );
}

#[test]
fn format_select_list_balances_across_more_than_two_lines() {
    // Same balancing, but with enough items that 4 lines are needed --
    // confirms the binary search in `balanced_fill_breaks` isn't just a
    // 2-line special case: 21/20/20/19 items per line, not a lopsided
    // greedy pack.
    assert_fmt!(
        r#"--
select
	c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21,
	c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41,
	c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59, c60, c61,
	c62, c63, c64, c65, c66, c67, c68, c69, c70, c71, c72, c73, c74, c75, c76, c77, c78, c79, c80
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
group by
	really_long_column_name_one, really_long_column_name_two, really_long_column_name_three, really_long_column_name_four
"#
    );
}

#[test]
fn format_order_by_wraps_when_too_long() {
    // Balanced across its two lines rather than cramming 3 items onto the
    // first line and leaving a lone 4th on the second.
    assert_fmt!(
        r#"--
select a
from t
order by
	really_long_column_name_one desc, really_long_column_name_two desc,
	really_long_column_name_three desc, really_long_column_name_four desc
"#
    );
}

#[test]
fn format_from_items_packs_when_it_fits() {
    // The where clause forces the statement to break onto multiple lines,
    // but the short from-item list still packs onto its own single line
    // rather than exploding to one table per line.
    assert_fmt!(
        r#"--
select a
from t1, t2, t3
where really_long_condition_one_name = 1
	and really_long_condition_two_name = 2
	and really_long_condition_three_name = 3
"#
    );
}

#[test]
fn format_from_items_wrap_when_too_long() {
    assert_fmt!(
        r#"--
select a
from
	really_long_table_name_one, really_long_table_name_two, really_long_table_name_three,
	really_long_table_name_four, really_long_table_name_five
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
    // All 4 arguments here are "simple" (bare names), so SqlExpressionList's
    // fill layout packs them together rather than one per line -- see
    // format_function_call_arguments_fill_packs_simple_and_breaks_complex
    // below for the mixed simple/complex case.
    assert_fmt_node!(
        "select a from generate_series(really_long_argument_one, really_long_argument_two, really_long_argument_three, really_long_argument_four) g",
        SqlSyntaxKind::SQL_FUNCTION_BINDING,
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
        SqlSyntaxKind::SQL_FUNCTION_BINDING,
        "generate_series(\n\treally_long_argument_one, really_long_argument_two,\n\tcoalesce(x, y, z),\n\treally_long_argument_four, really_long_argument_five\n) g"
    );
}
