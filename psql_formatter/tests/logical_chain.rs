#[macro_use]
mod helper;

use psql_syntax::PsqlSyntaxKind;

#[test]
fn format_two_conditions_stays_flat() {
    assert_fmt_node!(
        "select a from t where a > 1 and b < 2",
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        "a > 1 and b < 2"
    );
}

#[test]
fn format_two_conditions_stays_flat_even_when_long() {
    // Style rule 5: only condition *count* matters, not line width -- two
    // conditions never wrap, no matter how long.
    assert_fmt_node!(
        "select a from t where really_long_column_name_one > 1 and really_long_column_name_two < 2",
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        "really_long_column_name_one > 1 and really_long_column_name_two < 2"
    );
}

#[test]
fn format_three_conditions_wraps_with_leading_operator() {
    assert_fmt_node!(
        "select a from t where a > 1 and b < 2 and c = 3",
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        "a > 1\n\tand b < 2\n\tand c = 3"
    );
}

#[test]
fn format_or_chain_wraps_the_same_way() {
    assert_fmt_node!(
        "select a from t where a = 1 or a = 2 or a = 3",
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        "a = 1\n\tor a = 2\n\tor a = 3"
    );
}

#[test]
fn format_mixed_and_or_only_flattens_same_operator() {
    // `and` binds tighter than `or`, so this parses as `(a and b) or c` --
    // the `and` sub-chain (2 operands) and `c` together form a 2-operand
    // `or` chain, so nothing here wraps.
    assert_fmt_node!(
        "select x from t where a and b or c",
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        "a and b or c"
    );
}

#[test]
fn format_where_clause_with_wrapped_condition() {
    assert_fmt!(
        r#"--
select a
from t
where a > 1
	and b < 2
	and c = 3
"#
    );
}

#[test]
fn format_join_on_condition_wraps_when_more_than_two() {
    assert_fmt!(
        r#"--
select a
from t1
join t2 on t1.id = t2.id
	and t1.x = t2.x
	and t1.y = t2.y
"#
    );
}
