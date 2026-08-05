#[macro_use]
mod helper;

#[test]
fn format_redundant_parens_around_a_bare_name_are_removed() {
    assert_fmt_eq!(
        r#"--
select (a)
"#,
        r#"--
select a
"#
    );
}

#[test]
fn format_redundant_nested_parens_are_removed() {
    assert_fmt_eq!(
        r#"--
select ((a))
"#,
        r#"--
select a
"#
    );
}

#[test]
fn format_semantically_necessary_parens_around_a_binary_operand_are_preserved() {
    // `(a + b) * c` -- dropping the parens would change the result (`*`
    // binds tighter than `+`). The removal pass strips them like any
    // other parenthesized expression, but `SqlBinaryExpression`'s
    // `NeedsParentheses` impl re-inserts them, since the (now direct)
    // parent `*` binds tighter than the `+` child.
    assert_fmt!(
        r#"--
select (a + b) * c
"#
    );
}

#[test]
fn format_semantically_necessary_parens_around_a_right_operand_are_preserved() {
    // `a - (b - c)` -- `-` isn't associative, so the parens are load
    // bearing. Same precedence-based re-insertion as above, but via the
    // same-precedence-right-operand rule rather than a strict precedence
    // difference.
    assert_fmt!(
        r#"--
select a - (b - c)
"#
    );
}

#[test]
fn format_redundant_parens_around_mixed_and_or_are_kept_as_readability_parens() {
    // `(a and b) or c` -- semantically unnecessary (`and` already binds
    // tighter), but kept for readability by `SqlLogicalExpression`'s
    // `NeedsParentheses` impl whenever `and`/`or` mix without an explicit
    // grouping.
    assert_fmt!(
        r#"--
select a where (a and b) or c
"#
    );
}
