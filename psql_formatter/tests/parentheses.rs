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
    // binds tighter than `+`). `PsqlBinaryExpression` doesn't have its own
    // real formatter yet (still `format_verbatim_node`), so this is
    // currently preserved via the transform's source map recovering the
    // original, paren-inclusive text for the whole outer expression --
    // not yet via `NeedsParentheses` re-insertion (that's still to come).
    assert_fmt!(
        r#"--
select (a + b) * c
"#
    );
}

#[test]
fn format_semantically_necessary_parens_around_a_right_operand_are_preserved() {
    // `a - (b - c)` -- `-` isn't associative, so the parens are load
    // bearing. Same "still verbatim, recovered via the source map" note as
    // above.
    assert_fmt!(
        r#"--
select a - (b - c)
"#
    );
}
