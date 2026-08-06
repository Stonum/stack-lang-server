#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// `DISTINCT ON` and `LIMIT` are both Postgres-only.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_select_literal_list() {
    let res = parse(
        "select *, 1, 2, true, false, 'str', 1.15",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_table_qualified_star() {
    let res = parse("select t.* from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_aliased_table_qualified_star() {
    let res = parse("select a.* from t a", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_schema_qualified_star() {
    let res = parse("select s.t.* from s.t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_bare_and_table_qualified_star_together() {
    let res = parse("select *, t.a, u.* from t, u", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_table_qualified_star_with_other_columns() {
    let res = parse(
        "select t.*, u.a, u.b from t join u on t.id = u.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_dotted_column_reference_not_confused_with_table_star() {
    // `t.a` (an ordinary qualified column reference) must still parse as
    // before -- only a trailing `.*` triggers the table-star path.
    let res = parse("select t.a from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_asliased_literals() {
    let res = parse(
        "select 1 as x, 2 as y, 3 z, 4 \"as\"",
        SqlFileSource::script(),
    );

    assert_parser!(res, debug);
}

#[test]
fn test_select_binary_expressions() {
    let res = parse(
        "select 1 + 2, 3 - 4, 5 * 6, 7 / 8, 9 % 10, 1 < 2, 1 <= 2, 1 > 2, 1 >= 2, 1 = 2, 1 != 2, 1 <> 2",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_logical_expressions() {
    let res = parse(
        "select a and b, a or b, a and b or c",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_binary_expression_precedence() {
    let res = parse(
        "select 1 + 2 * 3, (1 + 2) * 3, a or b and c, a = 1 and b = 2",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_concat_expression() {
    let res = parse("select a || b", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_chained_concat_expression() {
    let res = parse(
        "select a || ', ' || b || ', ' || c",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_concat_binds_tighter_than_comparison() {
    // `||` sits above comparisons in precedence (real Postgres's "any other
    // operator" tier), so `a || b = c` is `(a || b) = c`, not `a || (b =
    // c)` -- the latter would be a type error in real Postgres anyway
    // (comparing text to a boolean).
    let res = parse("select a || b = c", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_concat_binds_looser_than_additive() {
    // `+`/`-` bind tighter than `||`, so `a + b || c` is `(a + b) || c`.
    let res = parse("select a + b || c", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_concat_inside_case_expression() {
    // Representative of a real shape: `||` chained across `case when`
    // branches to build a display string.
    let res = parse(
        "select case when a is not null then a || ', ' else '' end || case when b is not null then b || ', ' else '' end from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_json_field_operator() {
    let res = parse("select data -> 'a' from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_json_text_operator() {
    let res = parse("select data ->> 'a' from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_chained_json_operators() {
    let res = parse("select data -> 'a' ->> 'b' from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_json_operator_no_spaces() {
    let res = parse("select data->'a'->>'b' from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_json_operator_binds_tighter_than_comparison() {
    let res = parse(
        "select a from t where data -> 'a' = 'x'",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_minus_and_comment_still_work_after_arrow_operator() {
    // Regression test: `-` and `--` (line comments) must still lex
    // correctly now that `read_minus` also checks for a following `>`.
    let res = parse(
        "-- a comment\nselect 1 - 2, a - -b from t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_col_references() {
    let res = parse("select a, t.a, s.t.a, d.s.t.a", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_col_reference_with_implicit_alias() {
    let res = parse("select t.a b, t.a as b", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_from_table() {
    let res = parse("select a from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_from_qualified_table_with_alias() {
    let res = parse(
        "select t.a from mydb.myschema.mytable as t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_from_function_binding() {
    let res = parse(
        "select a from some_func(1, 2) as t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_from_qualified_function_binding() {
    let res = parse(
        "select a from myschema.some_func(1, x) t",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_where_clause() {
    let res = parse(
        "select a from t where a = 1 and b > 2",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_where_clause_without_from() {
    let res = parse("select 1 where true", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_group_by_clause() {
    let res = parse("select a, b from t group by a, b,", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_having_clause() {
    let res = parse(
        "select a, b from t group by a having b > 1",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_order_by_clause() {
    let res = parse(
        "select a, b from t order by a asc, b desc, a + b",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_order_by_after_having() {
    let res = parse(
        "select a, b from t group by a having b > 1 order by a desc",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_limit_offset() {
    let res = parse("select a from t limit 10 offset 20", postgres());

    assert_parser!(res);
}

#[test]
fn test_select_limit_offset_after_order_by() {
    let res = parse("select a from t order by a limit 10 offset 20", postgres());

    assert_parser!(res);
}

#[test]
fn test_select_distinct() {
    let res = parse("select distinct a, b from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_all() {
    let res = parse("select all a, b from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_distinct_on() {
    let res = parse("select distinct on (a, b) a, b, c from t", postgres());

    assert_parser!(res);
}

#[test]
fn test_select_distinct_on_single_column() {
    let res = parse(
        "select distinct on (a) a, b from t order by a, b",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_distinct_star() {
    let res = parse("select distinct * from t", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_trailing_semicolon() {
    let res = parse("select a from t;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_select_order_by_irregular_case_and_spacing() {
    let res = parse(
        "select a from t  ORDER   BY  a desc",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_bare_order_as_column_name_still_works() {
    let res = parse(
        "select order from t where order = 1",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_fused_underscore_order_by_is_rejected() {
    // `order_by` (one word) isn't real Postgres syntax -- only the real
    // two-word `order by` is accepted (see the tests above).
    let res = parse("select a from t order_by a", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_multiple_select_statements() {
    let res = parse(
        "select a from t; select b from u where b > 1;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_multiple_select_statements_without_trailing_semicolon() {
    let res = parse("select a from t; select b from u", SqlFileSource::script());

    assert_parser!(res);
}
