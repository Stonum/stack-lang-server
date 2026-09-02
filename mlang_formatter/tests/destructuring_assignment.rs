#[macro_use]
mod helper;

#[test]
fn format_array_destructuring_assignment() {
    assert_fmt!(
        r#"#
@[a, b, c] = myarr;
"#
    );
}

#[test]
fn format_array_destructuring_assignment_with_holes() {
    assert_fmt!(
        r#"#
@[a, _, c] = myarr;
"#
    );
}

#[test]
fn format_array_destructuring_assignment_with_rest() {
    assert_fmt!(
        r#"#
@[a, ...rest] = myarr;
"#
    );
}

#[test]
fn format_array_destructuring_assignment_with_member_targets() {
    assert_fmt!(
        r#"#
@[a.b, c[0]] = myarr;
"#
    );
}

#[test]
fn format_object_destructuring_assignment_shorthand() {
    assert_fmt!(
        r#"#
@{a, b} = myobj;
"#
    );
}

#[test]
fn format_object_destructuring_assignment_renamed() {
    assert_fmt!(
        r#"#
@{b: thisY, a: thisX} = myobj;
"#
    );
}

#[test]
fn format_object_destructuring_assignment_with_rest() {
    assert_fmt!(
        r#"#
@{y: thisY, A, ...s} = myobj;
"#
    );
}

#[test]
fn format_object_destructuring_assignment_empty() {
    assert_fmt!(
        r#"#
@{} = myobj;
"#
    );
}

#[test]
fn format_array_destructuring_assignment_wraps_long_content_without_trailing_comma_after_rest() {
    assert_fmt!(
        r#"#
@[
   значениеПервоеДлинное,
   значениеВтороеДлинное,
   значениеТретьеДлинное,
   значениеЧетвертоеДлинное,
   значениеПятоеДлинное,
   ...остальныеЗначения
] = myarr;
"#
    );
}

#[test]
fn format_object_destructuring_assignment_wraps_long_content_without_trailing_comma_after_rest() {
    assert_fmt!(
        r#"#
@{
   значениеПервоеДлинное,
   значениеВтороеДлинное,
   значениеТретьеДлинное,
   значениеЧетвертоеДлинное,
   ...остальныеЗначения
} = myobj;
"#
    );
}

#[test]
fn format_destructuring_assignment_reformats_badly_formatted_input() {
    // Sanity-check the round-trip tests above aren't trivially passing: feed
    // mis-formatted input (extra spaces, wrong line breaks) and confirm the
    // formatter actually normalizes it, rather than just accepting
    // already-correct input unchanged.
    assert_fmt_eq!(
        r#"@[a,
   b,c]   =   myarr;"#,
        "@[a, b, c] = myarr;"
    );

    assert_fmt_eq!(
        r#"@{a:   x  ,
b}=myobj;"#,
        "@{a: x, b} = myobj;"
    );
}
