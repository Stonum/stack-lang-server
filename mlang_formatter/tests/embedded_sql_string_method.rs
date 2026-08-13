#[macro_use]
mod helper;

#[test]
fn format_call_reformats_the_string() {
    assert_fmt_eq!(
        r#"#
var qq = Query("select   *   from   t".format(this.x), 1);
"#,
        r#"#
var qq = Query("select * from t".format(this.x), 1);"#
    );
}

#[test]
fn any_method_name_works_not_just_format() {
    assert_fmt_eq!(
        r#"#
var qq = Query("select   *   from   t".trim(), 1);
"#,
        r#"#
var qq = Query("select * from t".trim(), 1);"#
    );
}

#[test]
fn preserves_repeated_format_placeholders() {
    // Deliberately messy spacing: proves this actually goes through
    // reformatting (and placeholder substitute/restore) rather than
    // coincidentally already matching -- see
    // `placeholder_between_join_and_where_leaves_the_string_untouched` below
    // for why that distinction matters here.
    assert_fmt_eq!(
        r#"#
var qq = Query("select   *   from   t   where   a = {0}   and   b={0}".format(this.x), 1);
"#,
        r#"#
var qq = Query("select * from t where a = {0} and b = {0}".format(this.x), 1);"#
    );
}

#[test]
fn falls_back_to_verbatim_on_invalid_sql() {
    assert_fmt!(
        r#"#
var qq = Query("not valid   sql !!!".format(this.x), 1);
"#
    );
}

#[test]
fn argument_position_does_not_gate_sql_detection() {
    // Detection is content-based, not position-based.
    assert_fmt_eq!(
        r#"#
var qq = Query(1, "select   *   from   t".format(this.x));
"#,
        r#"#
var qq = Query(1, "select * from t".format(this.x));"#
    );
}

#[test]
fn call_name_does_not_gate_sql_detection() {
    // Same as `argument_position_does_not_gate_sql_detection`, for the old
    // `sql_call_names` allowlist.
    assert_fmt_eq!(
        r#"#
var qq = some_other_function("select   *   from   t".format(this.x), 1);
"#,
        r#"#
var qq = some_other_function("select * from t".format(this.x), 1);"#
    );
}

#[test]
fn placeholder_between_join_and_where_leaves_the_string_untouched() {
    // `{0}` between a JOIN's `ON` and `WHERE` isn't a valid position, so
    // substitution fails to parse and the string stays untouched. The
    // surrounding call arguments still get ordinary formatting, though.
    // Deliberately messy spacing to prove the string isn't reformatted.
    assert_fmt_eq!(
        r#"#
var q = Query(`select   a   from ~t~ nl join #tmp as tf on nl.id=tf.id and tf.type=0   {0}   where nl.a = :ls`.format(this.filter), 1);
"#,
        r#"#
var q = Query(
   `select   a   from ~t~ nl join #tmp as tf on nl.id=tf.id and tf.type=0   {0}   where nl.a = :ls`.format(
      this.filter
   ),
   1
);"#
    );
}

#[test]
fn comment_is_preserved_while_the_string_still_reformats() {
    // A leading comment no longer suppresses SQL reformatting.
    assert_fmt_eq!(
        r#"#
var qq = Query(
   # leading comment
   "select   *   from   t".format(this.x),
   1
);
"#,
        r#"#
var qq = Query(
   # leading comment
   "select * from t".format(this.x),
   1
);"#
    );
}
