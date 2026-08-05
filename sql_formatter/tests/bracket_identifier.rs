#[macro_use]
mod helper;

use biome_formatter::LineWidth;
use sql_formatter::{BracketIdentifierStyle, SqlFormatOptions, format_node};
use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// Formats `src` under an explicit [BracketIdentifierStyle] and asserts the
/// result equals `dest`, also checking idempotency (same spirit as
/// `assert_fmt_eq!`, which can't be used here since it doesn't expose a way
/// to override `SqlFormatOptions`).
fn assert_fmt_with_bracket_style(src: &str, dest: &str, style: BracketIdentifierStyle) {
    let syntax = SqlFileSource::query()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true);
    let tree = parse(src, syntax);
    assert!(
        !tree.has_errors(),
        "parse errors for {:?}: {:?}",
        src,
        tree.diagnostics()
    );

    let options = || {
        SqlFormatOptions::new(syntax)
            .with_line_width(LineWidth::try_from(120).unwrap())
            .with_bracket_identifier_style(style)
    };
    let result = format_node(options(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();
    assert_eq!(
        dest, result,
        "input:\n======\n{}\n======\nformatted:\n======\n{}\n======\n",
        src, result
    );

    let tree2 = parse(&result, syntax);
    assert!(
        !tree2.has_errors(),
        "formatted output failed to reparse: {:?}",
        tree2.diagnostics()
    );
    let result2 = format_node(options(), &tree2.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();
    assert_eq!(
        result, result2,
        "formatting is not idempotent:\nfirst pass:\n======\n{}\n======\nsecond pass:\n======\n{}\n======\n",
        result, result2
    );
}

#[test]
fn format_bracket_identifier_style_preserve_keeps_brackets_as_written() {
    // `Preserve` is the default (a formatter shouldn't silently rewrite
    // identifier-quoting style unless asked) -- exercised explicitly here
    // regardless, so this test still demonstrates the option itself works
    // even if the default ever changes.
    assert_fmt_with_bracket_style(
        "select [Col-Name], [Col2] from t\n",
        "select [Col-Name], [Col2] from t\n",
        BracketIdentifierStyle::Preserve,
    );
}

#[test]
fn format_bracket_identifier_style_convert_to_quotes_in_select_list() {
    // `ConvertToQuotes` is `mlang_formatter`'s own choice (see its embedding
    // call site) for normalizing legacy `[bracket]`-quoted mlang queries to
    // Postgres's own `"..."` spelling.
    assert_fmt_with_bracket_style(
        "select [Col-Name], [Col2] from t;\n",
        "select \"Col-Name\", \"Col2\" from t;\n",
        BracketIdentifierStyle::ConvertToQuotes,
    );
}

#[test]
fn format_bracket_identifier_convert_to_quotes_in_where_clause() {
    assert_fmt_with_bracket_style(
        "select a from t where [Col-Name] = 1;\n",
        "select a from t where \"Col-Name\" = 1;\n",
        BracketIdentifierStyle::ConvertToQuotes,
    );
}

#[test]
fn format_dotted_bracket_identifier_convert_to_quotes() {
    assert_fmt_with_bracket_style(
        "select t.[Col Name] from t;\n",
        "select t.\"Col Name\" from t;\n",
        BracketIdentifierStyle::ConvertToQuotes,
    );
}

#[test]
fn format_bracket_identifier_convert_to_quotes_as_table_qualifier() {
    assert_fmt_with_bracket_style(
        "select a from [Table Name].t;\n",
        "select a from \"Table Name\".t;\n",
        BracketIdentifierStyle::ConvertToQuotes,
    );
}

#[test]
fn format_bracket_identifier_convert_to_quotes_in_insert_column_list() {
    assert_fmt_with_bracket_style(
        "insert into t ([Col-Name], [Col2]) values (1, 2);\n",
        "insert into t (\"Col-Name\", \"Col2\")\nvalues (1, 2);\n",
        BracketIdentifierStyle::ConvertToQuotes,
    );
}

#[test]
fn format_bracket_identifier_convert_to_quotes_in_update_set_clause() {
    assert_fmt_with_bracket_style(
        "update t set [Col-Name] = 1 where id = 2;\n",
        "update t\nset \"Col-Name\" = 1\nwhere id = 2;\n",
        BracketIdentifierStyle::ConvertToQuotes,
    );
}

#[test]
fn format_bracket_identifier_preserve_leaves_dotted_and_qualifier_forms_untouched() {
    // Regression coverage for `Preserve` across the same structural
    // positions `ConvertToQuotes` is exercised in above -- not just the
    // single-position test at the top of this file.
    assert_fmt_with_bracket_style(
        "select t.[Col Name] from [Table Name].t;\n",
        "select t.[Col Name] from [Table Name].t;\n",
        BracketIdentifierStyle::Preserve,
    );
}

#[test]
fn format_already_double_quoted_identifier_unaffected() {
    // Regression guard: an identifier that was already Postgres-style
    // double-quoted (not bracket-quoted) must round-trip unchanged
    // regardless of bracket style -- only bracket-quoted identifiers are
    // ever touched by `BracketIdentifierStyle`.
    assert_fmt!(
        r#"--
select "already-quoted" from t
"#
    );
}

#[test]
fn format_array_subscript_unaffected_by_bracket_identifier_support() {
    assert_fmt!(
        r#"--
select arr[1] from t
"#
    );
}

#[test]
fn format_array_type_suffix_unaffected_by_bracket_identifier_support() {
    assert_fmt!(
        r#"--
create table t (a int[]);
"#
    );
}
