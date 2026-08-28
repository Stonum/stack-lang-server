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
    // in `embedded_sql.rs`, but for a `+`-chain.
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
    // a separator would be wrong. Left as an ordinary `+`-chain, but still
    // hugged to the opening paren like any other multi-line-string-bearing
    // argument (see `contains_multi_line_string_token`'s doc comment in
    // `call_arguments.rs`): the generic (non-hugging) call-argument
    // layout reformats the whole argument list at a narrower width in an
    // isolated pass and re-splices it back line by line, which used to
    // bake the surrounding ambient indent inside these literals' own
    // embedded newlines -- invisible in this single-format assertion since
    // the call sits at the top level (zero ambient indent to bake in), but
    // growing without bound on every reformat pass once nested one level
    // deeper (e.g. inside an `if`). Two multi-line operands in one chain
    // (`` `...update` `` and `` `...set a = 1...` ``) used to be out of
    // scope for `try_format_flat_multiline_concatenation`'s flat rendering
    // (one multi-line operand only) and so always broke onto one operand
    // per line regardless of fit -- now that it handles any number of
    // them, this chain -- which fits well within the line width -- stays
    // flat instead.
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
fn concatenation_reformats_regardless_of_call_name() {
    // Detection is content-based, not callee-name-based -- unlike the old
    // `sql_call_names` allowlist.
    assert_fmt_eq!(
        r#"#
var qq = some_other_function("select   *   from   t   where   a   =   '" + x + "'", 1);
"#,
        r#"#
var qq = some_other_function("select * from t where a = '" + x + "'", 1);"#
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
    // Even cleanly separated by whitespace, a bare identifier isn't a valid
    // clause between a JOIN's `ON` and `WHERE`. Left untouched; the
    // surrounding call-argument list still gets ordinary formatting.
    // Deliberately messy spacing to prove the pieces aren't reformatted.
    assert_fmt_eq!(
        r#"#
var qq = Query(`select   a   from t nl join u tf on nl.id=tf.id and tf.type=0   ` + extraClause + `   where nl.a = 1`, 1);
"#,
        r#"#
var qq = Query(
   `select   a   from t nl join u tf on nl.id=tf.id and tf.type=0   `
      + extraClause
      + `   where nl.a = 1`,
   1
);"#
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

#[test]
fn concatenation_that_bails_out_of_sql_detection_does_not_grow_indent_each_pass_when_nested() {
    // Regression test: same shape as
    // `concatenation_bails_out_when_a_hole_sits_flush_against_a_keyword`,
    // but nested one level deep (inside an `if`) instead of sitting at the
    // top level -- that's what actually exposes the bug the hugging fix
    // there addresses, since a top-level call has no ambient indent to
    // bake into the literals' embedded newlines in the first place.
    use mlang_formatter::{IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_node};
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    let src = "#\nif(test)\n{\n   var upd = Command(`\n     update` + tableName + `\n        set a = 1\n  `, 0);\n}\n";

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
