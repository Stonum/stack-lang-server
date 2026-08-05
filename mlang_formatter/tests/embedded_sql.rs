#[macro_use]
mod helper;

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
fn embedded_sql_concatenation_is_left_untouched() {
    // The first argument isn't a single string literal (it's built via
    // concatenation), so it never reaches the SQL-reformat path at all --
    // deliberately out of scope (see project roadmap notes), not a bug.
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
fn non_allowlisted_call_with_string_argument_is_left_untouched() {
    // "some_other_function" isn't in the default `sql_call_names`, so its
    // string argument formats as an ordinary string literal, not SQL.
    assert_fmt!(
        r#"#
var qq = some_other_function(`select   *   from t`, 1);
"#
    );
}
