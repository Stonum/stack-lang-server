#[macro_use]
mod helper;

// These cover `format_binary_like_expression.rs`'s
// `try_format_flat_multiline_concatenation`: a `+` chain that embeds a
// multi-line string literal used to have every operand pushed onto its own
// line (via `biome_formatter`'s `Document::propagate_expand`, which
// unconditionally expands any group containing a raw embedded `\n`,
// regardless of whether the content before it actually needs to wrap). When
// the chain fits, it should instead stay on one line, with the embedded
// newline printing through as-is.

#[test]
fn multiline_operand_last_stays_flat() {
    assert_fmt!(
        r#"#
var x = "строка" + ппп + "строка 1
    строка 2";
"#
    );
}

#[test]
fn multiline_operand_with_long_ident_last_stays_flat() {
    // The ambient indent from the two enclosing `if`s (6 columns) plus
    // `var x = ` (8 columns) counts toward the 120-column budget too -- this
    // fixture's first line is 116 columns including that prefix, just under
    // the limit.
    assert_fmt!(
        r#"#
if(true)
   if(true)
      var x = "строка" + ппп + "строка 12345 строка 12345 строка 12345 строка 12345 строка 12345 строка 12345 строка
         строка 12345 строка 12345 строка 12345 строка 12345 строка 12345";
"#
    );
}

#[test]
fn multiline_operand_breaks_when_ambient_indent_pushes_it_over_width() {
    // Same shape as `multiline_operand_with_long_ident_last_stays_flat`, one
    // more "строка 12345 " repetition -- 129 columns including the ambient
    // indent, over the 120-column budget, so it must fall back to the
    // regular per-operand break instead of staying flat.
    assert_fmt!(
        r#"#
if(true)
   if(true)
      var x = "строка"
         + ппп + "строка 12345 строка 12345 строка 12345 строка 12345 строка 12345 строка 12345 строка 12345 строка
         строка 12345 строка 12345 строка 12345 строка 12345 строка 12345";
"#
    );
}

#[test]
fn multiline_operand_first_stays_flat() {
    assert_fmt!(
        r#"#
var x = "строка 1
    строка 2" + ппп + "строка";
"#
    );
}

#[test]
fn multiline_operand_middle_stays_flat() {
    assert_fmt!(
        r#"#
var x = ппп + "строка 1
    строка 2" + qqq;
"#
    );
}

#[test]
fn reformats_mis_formatted_multiline_concatenation() {
    assert_fmt_eq!(
        "var x=\"строка\"+ппп+\"строка 1 \n    строка 2\";\n",
        "var x = \"строка\" + ппп + \"строка 1 \n    строка 2\";"
    );
}

#[test]
fn short_concatenation_without_multiline_operand_is_unaffected() {
    assert_fmt!(
        r#"#
var x = "вввввв" + 123 + ппп + "123123";
"#
    );
}

#[test]
fn falls_back_to_per_operand_break_when_trailing_content_does_not_fit() {
    assert_fmt!(
        r#"#
var x =
   "a"
   + "line1
line2"
   + ааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааааа;
"#
    );
}

#[test]
fn mixed_plus_minus_chain_with_multiline_operand_is_unaffected() {
    // `-` shares `+`'s operator precedence, so this chain gets flattened
    // together by `split_into_left_and_right_sides` -- the flat-join
    // optimization must not treat `-` as concatenation.
    assert_fmt!(
        r#"#
var x =
   "a"
   + "line1
line2"
   - b;
"#
    );
}
