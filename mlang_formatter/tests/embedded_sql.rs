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
    // but reached via content-based detection (the parser already
    // classified this string as SQL on its own -- no explicit selection
    // needed). Deliberately messy input (`a=1` etc.) to prove real
    // reformatting happens, not just idempotent whitespace matching.
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
fn non_allowlisted_call_with_string_argument_is_left_untouched() {
    // "some_other_function" isn't in the default `sql_call_names`, so its
    // string argument formats as an ordinary string literal, not SQL.
    assert_fmt!(
        r#"#
var qq = some_other_function(`select   *   from t`, 1);
"#
    );
}

#[test]
fn selection_triggers_sql_formatting_for_non_allowlisted_call_argument() {
    // `some_other_function` isn't in `sql_call_names` (see
    // `non_allowlisted_call_with_string_argument_is_left_untouched` above),
    // so its argument is never treated as SQL by the ordinary call-argument
    // heuristic. An explicit `textDocument/rangeFormatting` selection of
    // exactly this string must still trigger SQL formatting, independent of
    // the callee's name and of argument position.
    let src = r#"#
var qq = some_other_function("select   *   from   t");
"#;
    let start = src.find('"').unwrap() as u32;
    let end = src.rfind('"').unwrap() as u32 + 1;

    assert_fmt_range!(
        src,
        "#\nvar qq = some_other_function(\"select * from t\");",
        start..end
    );
}

#[test]
fn selection_does_not_widen_ordinary_whole_document_formatting() {
    // Regression guard for the opposite direction: the very same string
    // that `selection_triggers_sql_formatting_for_non_allowlisted_call_argument`
    // reformats as SQL under an explicit selection must NOT be treated as
    // SQL during ordinary whole-document formatting (no `selected_range` is
    // ever set in that path) -- it only gets the plain literal-string
    // whitespace cleanup, same as
    // `non_allowlisted_call_with_string_argument_is_left_untouched`.
    assert_fmt_eq!(
        r#"#
var qq = some_other_function("select   *   from   t");
"#,
        r#"#
var qq = some_other_function("select   *   from   t");"#
    );
}

#[test]
fn whole_document_range_formatting_does_not_widen_sql_detection() {
    // A real `textDocument/rangeFormatting` request for "format the whole
    // document" (no dedicated whole-document endpoint exists -- clients
    // just pass a range spanning the entire file, see `lsp/src/format.rs`)
    // sets `selected_range` to that entire span. It must not be confused
    // with a genuine partial selection of this one string: the string's own
    // token range is a small subset of the whole-file range, not the other
    // way around, so `is_explicitly_selected`'s containment check correctly
    // stays `false` here.
    let src = r#"#
var qq = some_other_function("select   *   from   t");
"#;

    assert_fmt_range!(
        src,
        "#\nvar qq = some_other_function(\"select   *   from   t\");",
        0..(src.len() as u32)
    );
}
