#[macro_use]
mod helper;

#[test]
fn format_two_select_statements_are_separated_by_a_line_break() {
    assert_fmt!(
        r#"--
select 1;
select 2;
"#
    );
}

#[test]
fn format_drop_and_create_are_separated_by_a_line_break() {
    assert_fmt!(
        r#"--
drop function if exists foo;
create function foo() as 'select 1'
"#
    );
}

#[test]
fn format_go_batch_separator_recovers_without_swallowing_the_next_statement() {
    // `GO` isn't a real grammar node -- it falls into bogus-statement
    // recovery and is printed verbatim (so `has_errors()` is legitimately
    // true here, unlike `assert_fmt!`'s usual expectation). What matters is
    // that it doesn't also swallow the `CREATE FUNCTION` statement that
    // follows it (no intervening `;`): that statement must still get its
    // own, properly formatted node, on its own line.
    use biome_formatter::LineWidth;
    use sql_formatter::{SqlFormatOptions, format_node};
    use sql_parser::parse;
    use sql_syntax::{SqlDialect, SqlFileSource};

    let src = "drop function if exists foo;\nGO\ncreate function foo() as 'select 1';\nselect 1;\n";
    let syntax = SqlFileSource::query().with_dialect(SqlDialect::Mlang);
    let tree = parse(src, syntax);
    assert!(tree.has_errors());

    let options = SqlFormatOptions::new(syntax).with_line_width(LineWidth::try_from(120).unwrap());
    let result = format_node(options, &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();

    assert_eq!(
        result,
        "drop function if exists foo;\nGO\ncreate function foo() as 'select 1';\nselect 1;\n"
    );
}
