use mlang_formatter::{IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_node};
use mlang_parser::parse;
use mlang_syntax::MFileSource;

fn base_options(syntax: MFileSource) -> MFormatOptions {
    MFormatOptions::new(syntax)
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(120).unwrap())
        .with_indent_width(IndentWidth::from(3))
}

fn format_with_call_names(src: &str, sql_call_names: Vec<String>) -> String {
    let syntax = MFileSource::script();
    let tree = parse(src, syntax);
    let options = base_options(syntax).with_sql_call_names(sql_call_names);

    let doc = format_node(options, &tree.syntax()).unwrap();
    doc.print().unwrap().as_code().to_string()
}

#[test]
fn custom_wrapper_name_is_not_query_like_by_default() {
    let src = "var qq = my_wrapper(   `select a from t`   ,    1   );\n";

    // With the default `sql_call_names` (nothing overridden), "my_wrapper"
    // isn't recognized, so the call arguments format normally -- spacing
    // gets normalized.
    let syntax = MFileSource::script();
    let tree = parse(src, syntax);
    let doc = format_node(base_options(syntax), &tree.syntax()).unwrap();
    let formatted = doc.print().unwrap().as_code().to_string();

    assert_eq!(formatted, "var qq = my_wrapper(`select a from t`, 1);\n");
}

#[test]
fn custom_wrapper_name_is_query_like_when_configured() {
    let src = "var qq = my_wrapper(   `select a from t`   ,    1   );\n";

    // Adding "my_wrapper" to the configured list makes it query-like, so
    // its string argument is parsed and reformatted as SQL, same as the
    // built-in `query`/`command`/etc. names (the messy argument-list
    // spacing normalizes too, since the query parses cleanly -- see
    // `embedded_sql.rs` for the case where it *doesn't* parse and the
    // whole call stays verbatim instead).
    let formatted =
        format_with_call_names(src, vec!["my_wrapper".to_string(), "query".to_string()]);
    assert_eq!(formatted, "var qq = my_wrapper(`select a from t`, 1);\n");
}

#[test]
fn matching_is_case_insensitive() {
    let src = "var qq = MY_WRAPPER(   `select a from t`   ,    1   );\n";

    let formatted = format_with_call_names(src, vec!["my_wrapper".to_string()]);
    assert_eq!(formatted, "var qq = MY_WRAPPER(`select a from t`, 1);\n");
}

#[test]
fn custom_wrapper_name_reformats_embedded_sql_when_configured() {
    // Composing 5a (configurable detection) with 5b (real SQL reformat):
    // once "my_wrapper" is recognized, its string argument goes through
    // the exact same embedded-SQL parsing/formatting as the built-in names.
    let src = "var qq = my_wrapper(`select   *   from t`, 1);\n";

    let formatted = format_with_call_names(src, vec!["my_wrapper".to_string()]);
    assert_eq!(formatted, "var qq = my_wrapper(`select * from t`, 1);\n");
}
