#[macro_use]
mod helper;

#[test]
fn concatenation_with_hole_in_the_middle_reformats_each_literal_segment() {
    // The canonical dynamic-query shape: a literal, a hole, a literal --
    // each literal segment gets its own SQL reformatting independently,
    // and the hole (`userName`) is left as an ordinary mlang expression.
    assert_fmt!(
        r#"#
var qq = Query("select * from t where name = '" + userName + "'", 1);
"#
    );
}

#[test]
fn concatenation_collapses_extra_whitespace_in_every_literal_segment() {
    assert_fmt_eq!(
        r#"#
var qq = Query("select   id,   name   from   t   where   a   =   '" + a + "'   and   b   =   '" + b + "'", 1);
"#,
        r#"#
var qq = Query("select id, name from t where a = '" + a + "' and b = '" + b + "'", 1);"#
    );
}

#[test]
fn concatenation_hole_can_be_an_arbitrary_expression() {
    // Holes are opaque -- their own internal structure (here, a call
    // expression) is formatted normally via its own `Format` impl, not
    // reparsed as part of the SQL.
    assert_fmt!(
        r#"#
var qq = Query("select * from t where a = '" + escape(a) + "'", 1);
"#
    );
}

#[test]
fn concatenation_bare_assignment_keeps_operator_on_same_line_without_selection() {
    // Same shape as `embedded_sql_bare_assignment_keeps_operator_on_same_line_without_selection`
    // in `embedded_sql.rs`, but for a `+`-chain reclassified as
    // `MSqlConcatenationExpression` and assigned directly (not passed as a
    // call argument): `=` and the opening quote must stay on the same line,
    // not push the whole multi-line query onto a new, extra-indented line.
    assert_fmt_eq!(
        r#"#
if(test)
{
   var qq = "select a1,b2,c3,d4,e5,f6,g7 from " + table_name + " where a=1 and b=2 and c=3";
}
"#,
        r#"#
if(test)
{
   var qq = "
      select a1, b2, c3, d4, e5, f6, g7
      from " + table_name + "
      where a = 1
         and b = 2
         and c = 3
   ";
}"#
    );
}

#[test]
fn concatenation_picks_up_ambient_indent_in_nested_context() {
    assert_fmt!(
        r#"#
if(test)
{
   var qq = Query("select * from t where a = '" + a + "'", 1);
}
"#
    );
}

#[test]
fn concatenation_preserves_backtick_delimiter() {
    // Mlang's `` ` `` and `"` delimiters are interchangeable -- a chain
    // built from backtick-delimited literals stays backtick-delimited.
    assert_fmt!(
        r#"#
var qq = Query(`select * from t where a = '` + a + `'`, 1);
"#
    );
}

#[test]
fn concatenation_bails_out_on_adjacent_holes() {
    // Two holes with no literal between them can't be reconstructed (their
    // placeholders would merge into a single SQL token) -- the whole call
    // is left verbatim, same as a single query-like string that fails to
    // parse (see `call_arguments.rs`): we can't confidently reformat the
    // surrounding `+`-chain structure around content we don't understand.
    assert_fmt!(
        r#"#
var qq = Query(x + y, 1);
"#
    );
}

#[test]
fn concatenation_bails_out_on_adjacent_literals() {
    // Two literals with no hole between them are deliberately unsupported
    // too (see `flatten_concatenation_chain`'s doc comment) -- same
    // verbatim fallback.
    assert_fmt!(
        r#"#
var qq = Query("a" + "b" + x, 1);
"#
    );
}

#[test]
fn concatenation_bails_out_when_joined_text_does_not_parse_as_sql() {
    assert_fmt!(
        r#"#
var qq = Query("not valid   sql !!!" + x, 1);
"#
    );
}

#[test]
fn concatenation_bails_out_when_a_hole_sits_flush_against_a_keyword() {
    // A hole immediately adjacent to keyword text with no separator (no
    // space, no quote) merges with it into one identifier for the SQL
    // lexer (`update__mlang_hole_0__` isn't the `UPDATE` keyword anymore),
    // so the joined placeholder text fails to parse. This is ambiguous,
    // not necessarily a real bug: the *runtime* value of `tableName` might
    // itself start with a space or a quote and be perfectly valid SQL --
    // the source text alone can't tell us either way, and it's just as
    // plausible that the hole is meant to be glued onto a *partial
    // identifier* (e.g. `"prefix_" + suffix + "_table"`), where inserting
    // a separator would be wrong. Left verbatim rather than guessed at.
    assert_fmt!(
        r#"#
var upd = Command(`
     update` + tableName + `
        set a = 1
  `, 0);
"#
    );
}

#[test]
fn concatenation_is_left_untouched_for_non_allowlisted_call() {
    // "some_other_function" isn't in the default `sql_call_names`, so the
    // concatenation-formatting path never even attempts to engage.
    assert_fmt!(
        r#"#
var qq = some_other_function("select * from t where a = '" + x + "'", 1);
"#
    );
}

#[test]
fn concatenation_output_is_idempotent() {
    // Formatting already-formatted concatenation output must be a no-op --
    // the same guarantee every other formatted construct in this codebase
    // has.
    assert_fmt!(
        r#"#
var qq = Query("select id, name from t where a = '" + a + "' and b = '" + b + "'", 1);
"#
    );
}

#[test]
fn concatenation_bails_out_when_reformatting_would_split_across_lines() {
    // Regression test: `sql_formatter` breaking a long/multi-clause query
    // onto several lines (one per select/from/where/and clause) has no
    // reason to land exactly on a hole boundary. Here each hole sits inside
    // a still-open nested `'...'` string, so the piece it lands in ends (or
    // starts) with an unbalanced quote -- e.g. `"select a\nfrom t\nwhere x
    // = '"` followed by a hole, then `"'\n   and y = '"` for the next one.
    // `format_concatenation_chain` bails out on that specific shape
    // (unbalanced quotes in a multi-line piece, see `has_balanced_quotes`),
    // leaving the whole call verbatim instead (see `call_arguments.rs`).
    // Compare `concatenation_multiline_sql_with_holes_at_clause_boundaries`,
    // where holes sit at balanced-quote positions and DO get reformatted.
    assert_fmt!(
        r#"#
var qq = Query(
   "select a from t where x = '"
      + aaaaaaaaaaaaaaaaaaaa
      + "' and y = '"
      + bbbbbbbbbbbbbbbbbbbb
      + "' and z = '"
      + cccccccccccccccccccc
      + "'",
   1
);
"#
    );
}

#[test]
fn concatenation_bails_out_when_a_literal_has_a_comment() {
    // A comment attached to one of the chain's own literal/operator tokens
    // (as opposed to a hole, which is formatted normally and keeps its own
    // comments) would otherwise silently vanish, since
    // `FormatConcatenatedQuery` never calls those tokens' own `.format()`.
    // Bails out and leaves the whole call verbatim instead, which
    // trivially keeps the comment since nothing is touched at all.
    assert_fmt!(
        r#"#
var qq = Query(
   # leading comment
   "select * from t where a = '" + x + "'",
   1
);
"#
    );
}

#[test]
fn concatenation_bails_out_when_a_hole_sits_between_join_and_where() {
    // A hole isn't only opaque to the SQL grammar when it's flush against a
    // keyword (see `concatenation_bails_out_when_a_hole_sits_flush_against_a_keyword`)
    // -- even cleanly separated by whitespace, a bare identifier is not a
    // valid clause in the position between a JOIN's `ON` and `WHERE` (that's
    // a keyword/clause position, not an expression position). Left verbatim
    // on purpose: reformatting around content we can't parse risks
    // corrupting complex real-world queries, and this shape is rare enough
    // that leaving it to the user to format by hand is an acceptable
    // tradeoff. Deliberately messy spacing: if this ever silently
    // "succeeded" via ordinary reformatting instead of falling back, the
    // spacing would be cleaned up and this assertion would fail.
    assert_fmt!(
        r#"#
var qq = Query(`select   a   from t nl join u tf on nl.id=tf.id and tf.type=0   ` + extraClause + `   where nl.a = 1`, 1);
"#
    );
}

#[test]
fn concatenation_multiline_sql_with_holes_at_clause_boundaries() {
    assert_fmt!(
        r#"#
var qq = Query("
   select a1, b2, c3, d4, e5, f6, g7
   from " + function_name + "(1, 2, 3) f
   where true and " + filter_clause, 1);
"#
    );
}
