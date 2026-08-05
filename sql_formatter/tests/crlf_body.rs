#[macro_use]
mod helper;

use biome_formatter::LineWidth;
use sql_formatter::{SqlFormatOptions, format_node};
use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

/// A multi-line dollar-quoted body, kept verbatim by the formatter, can
/// still carry the source file's own line-ending style embedded directly
/// in its text (not just in between-token trivia, which is already
/// normalized elsewhere). On a CRLF source file this used to panic --
/// `biome_formatter`'s text builder asserts against raw `\r` in anything
/// printed via `dynamic_text`. Regression test: no panic, and the `\r`
/// is gone from the formatted output (Postgres doesn't care about
/// line-ending style inside a dollar-quoted string, so normalizing it is
/// harmless).
#[test]
fn format_crlf_dollar_quoted_body_does_not_panic() {
    let src = "create function foo() returns int as $$\r\nbegin\r\n  return 1;\r\nend;\r\n$$ language plpgsql;\n";
    let syntax = SqlFileSource::query()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true);
    let tree = parse(src, syntax);
    assert!(!tree.has_errors());

    let options = SqlFormatOptions::new(syntax).with_line_width(LineWidth::try_from(120).unwrap());
    let result = format_node(options, &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();

    assert!(
        !result.contains('\r'),
        "formatted output still contains a raw \\r: {result:?}"
    );
    assert_eq!(
        result,
        "create function foo() returns int as $$\nbegin\n  return 1;\nend;\n$$ language plpgsql;\n"
    );
}

#[test]
fn format_crlf_plain_string_literal_does_not_panic() {
    // A single-quoted string can also span multiple lines (real Postgres
    // allows embedded newlines inside `'...'`), so it's covered by the
    // same fix, not just dollar-quoted bodies.
    let src = "select 'line one\r\nline two' from t\n";
    let syntax = SqlFileSource::query()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true);
    let tree = parse(src, syntax);
    assert!(!tree.has_errors());

    let options = SqlFormatOptions::new(syntax).with_line_width(LineWidth::try_from(120).unwrap());
    let result = format_node(options, &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();

    assert!(!result.contains('\r'));
    assert_eq!(result, "select 'line one\nline two'\nfrom t\n");
}
