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
    // `placeholder_between_join_and_where_falls_back_to_verbatim` below for
    // why that distinction matters here.
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
fn does_not_trigger_when_not_the_first_argument() {
    assert_fmt!(
        r#"#
var qq = Query(1, "select   *   from   t".format(this.x));
"#
    );
}

#[test]
fn does_not_trigger_for_non_allowlisted_call() {
    assert_fmt!(
        r#"#
var qq = some_other_function("select   *   from   t".format(this.x), 1);
"#
    );
}

#[test]
fn placeholder_between_join_and_where_falls_back_to_verbatim() {
    // Real-world shape: `{0}` sits directly between a JOIN's `ON` clause and
    // `WHERE`, a position no bare identifier can occupy (it's not a
    // scalar/expression position) -- substitution parses to an error and the
    // whole call safely falls back to verbatim, no corruption. Deliberately
    // messy spacing: if this ever silently "succeeded" via ordinary
    // reformatting instead of falling back, the spacing would be cleaned up
    // and this assertion would fail.
    assert_fmt!(
        r#"#
var q = Query(`select   a   from ~t~ nl join #tmp as tf on nl.id=tf.id and tf.type=0   {0}   where nl.a = :ls`.format(this.filter), 1);
"#
    );
}

#[test]
fn falls_back_to_verbatim_when_a_comment_is_present() {
    assert_fmt!(
        r#"#
var qq = Query(
   # leading comment
   "select   *   from   t".format(this.x),
   1
);
"#
    );
}
