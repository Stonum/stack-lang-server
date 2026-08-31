#[macro_use]
mod helper;

use sql_syntax::SqlSyntaxKind;

#[test]
fn format_two_conditions_stays_flat() {
    assert_fmt_node!(
        "select a from t where a > 1 and b < 2",
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        "a > 1 and b < 2"
    );
}

#[test]
fn format_two_conditions_stays_flat_even_when_long() {
    // Style rule 5: only condition *count* matters, not line width -- two
    // conditions never wrap, no matter how long.
    assert_fmt_node!(
        "select a from t where really_long_column_name_one > 1 and really_long_column_name_two < 2",
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        "really_long_column_name_one > 1 and really_long_column_name_two < 2"
    );
}

#[test]
fn format_three_conditions_wraps_with_leading_operator() {
    assert_fmt_node!(
        "select a from t where a > 1 and b < 2 and c = 3",
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        "a > 1\n\tand b < 2\n\tand c = 3"
    );
}

#[test]
fn format_or_chain_wraps_the_same_way() {
    assert_fmt_node!(
        "select a from t where a = 1 or a = 2 or a = 3",
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        "a = 1\n\tor a = 2\n\tor a = 3"
    );
}

#[test]
fn format_mixed_and_or_only_flattens_same_operator() {
    // `and` binds tighter than `or`, so this parses as `(a and b) or c`
    // without needing any parens to preserve that grouping -- but the
    // formatter adds them anyway for readability whenever `and`/`or` mix
    // without an explicit grouping (see `NeedsParentheses` for
    // `SqlLogicalExpression`). The `and` sub-chain (2 operands) and `c`
    // together still form a 2-operand `or` chain, so nothing here wraps
    // onto multiple lines.
    assert_fmt_node!(
        "select x from t where a and b or c",
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        "(a and b) or c"
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
fn format_preserves_a_comment_trailing_a_flattened_intermediate_operand() {
    // A comment right after an `and`/`or` operator has no node of its own
    // to anchor to (see `handle_logical_expression_operator_comment`) --
    // it's placed as a leading comment of the operand that follows,
    // regardless of which line it originally shared. Here that's `c = 3`,
    // so the comment doesn't stop `a = 1 and b = 2` from flattening.
    assert_fmt_node!(
        "select a from t where a = 1 and b = 2 /* c */ and c = 3",
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        "a = 1\n\tand b = 2\n\tand /* c */ c = 3"
    );
}

#[test]
fn format_two_conditions_wraps_when_one_is_a_nested_group() {
    // A 2-operand `and` chain still counts as "at most two conditions",
    // but the second operand is itself an `or` whose own two operands are
    // each further `and` groups -- exactly the kind of hidden complexity
    // (logical-inside-logical, not just a long leaf like `between`) Style
    // rule 5's exemption must not paper over.
    assert_fmt!(
        r#"--
select a
from t
where a = 1
	and ((b between c and d and x = 1) or (e between f and g and y = 2))
"#
    );
}

#[test]
fn format_nested_group_of_leaves_stays_exempt() {
    // The nested group's own operands are plain leaves (a `between` and a
    // comparison, not a further `and`/`or`), so it's still a "simple"
    // operand and the outer 2-operand chain stays flat.
    assert_fmt!(
        r#"--
select a from t where a = 1 and (b between c and d or e = 2)
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
