#[macro_use]
mod helper;

#[test]
fn embedded_sql_preserves_format_style_placeholders() {
    assert_fmt_eq!(
        r#"#
var qq = Query("select   *   from   t   where   a   =   {0}", 1);
"#,
        r#"#
var qq = Query("select * from t where a = {0}", 1);"#
    );
}

#[test]
fn embedded_sql_preserves_repeated_format_style_placeholders() {
    assert_fmt!(
        r#"#
var qq = Query("select * from t where a = {0} and b = {0}", 1);
"#
    );
}

#[test]
fn embedded_sql_with_syntax_errors_falls_back_to_verbatim() {
    // The content doesn't parse as SQL at all -- must be left exactly as
    // written rather than risk corrupting it under a "prettier" guess.
    assert_fmt!(
        r#"#
var qq = Query(`this is not valid   sql at all !!!`, 1);
"#
    );
}

#[test]
fn embedded_sql_unparseable_multiline_argument_does_not_grow_indent_each_pass() {
    // Regression test: a call's sole argument that's a multi-line string
    // but doesn't parse as SQL (so it's left as an ordinary literal, not
    // hugged) used to go through `write_with_custom_line_width`, which
    // re-splices its own already-formatted text back in line by line via
    // `hard_line_break()` so the real document's ambient indent applies.
    // That swept up the string's own embedded newlines too, baking the
    // ambient indent *inside* the string's literal content -- so every
    // reformat pass added another layer on top of the last, permanently
    // growing the indent. Any multi-line string argument is now hugged
    // (`should_hug_first_call_argument`), sidestepping that path entirely.
    use mlang_formatter::{IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_node};
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    let src = "#\nvar x = EXEC_COMMAND(`\n      create table #tmp(\n         \"a\" int\n      );\n   `);\n";

    let syntax = MFileSource::script();
    let options = MFormatOptions::new(syntax)
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(120).unwrap())
        .with_pretty_line_width(LineWidth::try_from(90).unwrap())
        .with_indent_width(IndentWidth::from(3))
        .with_bracket_spacing(false.into());

    let tree = parse(src, syntax);
    let pass1 = format_node(options.clone(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .as_code()
        .to_string();

    let tree2 = parse(&pass1, syntax);
    let pass2 = format_node(options, &tree2.syntax())
        .unwrap()
        .print()
        .unwrap()
        .as_code()
        .to_string();

    assert_eq!(
        pass1, pass2,
        "formatting is not idempotent:\nfirst pass:\n======\n{pass1}\n======\nsecond pass:\n======\n{pass2}\n======\n"
    );
}

#[test]
fn embedded_sql_concatenation_with_trailing_hole_reformats_cleanly() {
    // The first argument isn't a single string literal -- it's built via
    // `+`-concatenation with a hole at the very end -- but that's exactly
    // the shape `utils/concatenation.rs` now understands: this input is
    // already idempotent (its spacing happens to match what
    // `sql_formatter` would produce). See `mlang_formatter/tests/embedded_sql_concatenation.rs`
    // for the dedicated concatenation-formatting test suite.
    assert_fmt!(
        r#"#
var qq = Query("select * from t where a = " + x, 1);
"#
    );
}

#[test]
fn embedded_sql_preserves_double_quote_delimiter() {
    // mlang's lexer treats `` ` `` and `"` identically (both support
    // literal embedded newlines), so a double-quoted query must stay
    // double-quoted -- not get forced into backticks.
    assert_fmt!(
        r#"#
var qq = Query("select row_id from t where a = :1", 1);
"#
    );
}

#[test]
fn embedded_sql_with_escaped_double_quote_falls_back_to_verbatim() {
    // A `"`-delimited mlang string can only contain a literal `"` as an
    // escaped `\"` (mlang's own lexer requires it -- an unescaped `"`
    // would have ended the string there). Since that backslash isn't
    // meaningful SQL syntax, this reliably fails to parse and safely falls
    // back to verbatim instead of corrupting the query -- a real,
    // documented limitation of quote-delimited (not backtick) embedded
    // queries containing quoted SQL identifiers.
    assert_fmt!(
        r#"#
var qq = Query("select \"Col\" from t", 1);
"#
    );
}

#[test]
fn embedded_sql_multi_line_indent_matches_top_level_context() {
    // The opening quote hugs the opening paren (rule 1), and the trailing
    // `1` argument stays inline after the closing quote since it fits
    // (rule 2) -- same "group the first argument" mechanism `foo(function
    // () {...}, other)` already uses.
    assert_fmt!(
        r#"#
var qq = Query(`
   select row_id
   from t
   where a = :1
      and b = :2
      and c = :3
`, 1);
"#
    );
}

#[test]
fn embedded_sql_bracket_identifier_switches_double_quoted_string_to_backtick() {
    // Reformatting canonicalizes mlang's `[bracket]` identifiers to
    // Postgres's own `"..."` spelling -- which would otherwise clash with
    // (and have to be escaped inside) the query string's own `"`
    // delimiter. Since mlang treats `` ` `` and `"` as interchangeable
    // string delimiters, switching to `` ` `` avoids that entirely.
    assert_fmt_eq!(
        r#"#
var qq = query("select [a] from [b]");
"#,
        r#"#
var qq = query(`select "a" from "b"`);"#
    );
}

#[test]
fn embedded_sql_backslash_escape_is_not_doubled_on_repeated_formatting() {
    // Regression test: a `\` in the query text (here, a regex literal)
    // already forms a valid two-character mlang escape sequence in the
    // raw source and must round-trip through `sql_formatter` and back into
    // the mlang string literal unchanged -- not gain an extra `\` on every
    // formatting pass.
    assert_fmt_eq!(
        r#"#
var qq = query(`select   1   where   x   ~   '\\d+'`);
"#,
        r#"#
var qq = query(`select 1 where x ~ '\\d+'`);"#
    );
    assert_fmt!(
        r#"#
var qq = query(`select 1 where x ~ '\\d+'`);
"#
    );
}

#[test]
fn embedded_sql_without_bracket_identifiers_keeps_double_quote_delimiter() {
    // No conflicting `"` introduced by reformatting -- the original `"`
    // delimiter must stay untouched, not switch to backtick unnecessarily.
    assert_fmt!(
        r#"#
var qq = query("select a from b");
"#
    );
}

#[test]
fn embedded_sql_backtick_delimited_bracket_identifier_stays_backtick() {
    // Already backtick-delimited -- the canonicalized `"..."` identifiers
    // never conflict with `` ` ``, so the delimiter is simply preserved
    // (only the bracket-to-quote identifier canonicalization changes
    // anything here).
    assert_fmt_eq!(
        r#"#
var qq = query(`select [a] from [b]`);
"#,
        r#"#
var qq = query(`select "a" from "b"`);"#
    );
}

#[test]
fn embedded_sql_multi_line_indent_matches_nested_context() {
    // The embedded query's own lines must pick up the *ambient* indent at
    // the point it's written, not always the top level -- here, one extra
    // level from being inside the `if` block.
    assert_fmt!(
        r#"#
if(test)
{
   var qq = Query(`
      select row_id
      from t
      where a = :1
         and b = :2
         and c = :3
   `, 1);
}
"#
    );
}

#[test]
fn embedded_sql_selection_on_bare_assignment_keeps_operator_on_same_line() {
    // Regression test: reformatting a plain string assigned directly (not
    // passed as a call argument) via an explicit `textDocument/rangeFormatting`
    // selection used to pick up an extra indent level, because
    // `AssignmentLikeLayout::BreakAfterOperator` (`assignment_like.rs`) forced
    // `=` onto its own line and indented the whole value once the string
    // became multi-line, stacking on top of the string's own internal
    // block-indent. The multi-line output must match the indentation of the
    // equivalent `Query(...)` call-argument case (see
    // `embedded_sql_multi_line_indent_matches_nested_context`): `=` and the
    // opening quote stay on the same line, one indent level for the SQL
    // body, one more for the `and` continuations.
    let src = r#"#
if(test)
{
   var qq = "select row_id from t where a = 1 and b = 2 and c = 3";
}
"#;
    let start = src.find('"').unwrap() as u32;
    let end = src.rfind('"').unwrap() as u32 + 1;

    assert_fmt_range!(
        src,
        r#"var qq = "
      select row_id
      from t
      where a = 1
         and b = 2
         and c = 3
   ";"#,
        start..end
    );
}

#[test]
fn embedded_sql_bare_assignment_keeps_operator_on_same_line_without_selection() {
    // Same regression as `embedded_sql_selection_on_bare_assignment_keeps_operator_on_same_line`,
    // reached via content-based detection instead of a selection.
    assert_fmt_eq!(
        r#"#
if(test)
{
   var qq = "select row_id from t where a=1 and b=2 and c=3";
}
"#,
        r#"#
if(test)
{
   var qq = "
      select row_id
      from t
      where a = 1
         and b = 2
         and c = 3
   ";
}"#
    );
}

#[test]
fn call_name_does_not_gate_sql_detection() {
    // Detection is content-based, not callee-name-based -- unlike the old
    // `sql_call_names` allowlist.
    assert_fmt_eq!(
        r#"#
var qq = some_other_function(`select   *   from t`, 1);
"#,
        r#"#
var qq = some_other_function(`select * from t`, 1);"#
    );
}

#[test]
fn argument_position_does_not_gate_sql_detection() {
    // Same as `call_name_does_not_gate_sql_detection`, for argument position.
    assert_fmt_eq!(
        r#"#
var qq = some_other_function(1, "select   *   from   t");
"#,
        r#"#
var qq = some_other_function(1, "select * from t");"#
    );
}
