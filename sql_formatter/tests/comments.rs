#[macro_use]
mod helper;

#[test]
fn format_comments_between_returns_and_as_are_stable() {
    // Regression test: a comment sitting between `returns ...` and `as`
    // (trivia of the bare `as` token, with no node of its own to anchor
    // to) used to get reclassified differently across a second format
    // pass -- reordered and merged onto one line -- because its default
    // placement depended on incidental same-line-vs-different-line
    // whitespace shape rather than tree structure. `assert_fmt!` already
    // checks a second pass reproduces the same output byte-for-byte.
    assert_fmt!(
        r#"--
create function foo(a int) returns int as -- comment one
-- comment two
$$select 1$$ language plpgsql;
"#
    );
}

#[test]
fn format_comment_between_parameters_and_returns_is_stable() {
    assert_fmt!(
        r#"--
create function foo(a int) -- a comment
returns int as $$select 1$$;
"#
    );
}
