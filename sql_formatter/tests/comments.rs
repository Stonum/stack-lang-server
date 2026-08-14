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

#[test]
fn format_comment_after_and_or_operator_is_stable() {
    // Same instability as the `create function` cases above, for a comment
    // right after an `and`/`or` operator inside a wrapped chain.
    use biome_formatter::{IndentStyle, IndentWidth};
    use sql_formatter::{SqlFormatOptions, format_node};
    use sql_parser::parse;
    use sql_syntax::{SqlDialect, SqlFileSource};

    let src = "Select sv.row_id\n                                  FROM t1 sv\n                                  join t2 vp on sv.\"a\" = vp.row_id and vp.\"b\" in ( 'X', 'Y', 'Z')\n                                  join t3 li on li.c = :1 and li.d = 4 and sv.\"e\" = li.f\n                                  where sv.g >= :2 and sv.h <= :3\n                                      -- some comment here about category values 1, 2(x) and 5(y)\n                                      and (sv.i in (1,2,5) and vp.\"b\" = 'CATEGORY' or true)\n                                   limit 1";

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
