//! Error-recovery: malformed input must not panic, must produce diagnostics,
//! and must stay lossless (the tree reproduces the source byte-for-byte).

use xml_parser::parse;

fn assert_lossless(src: &str) {
    let parsed = parse(src);
    assert_eq!(
        parsed.syntax().to_string(),
        src,
        "recovery is not lossless for {src:?}"
    );
}

#[test]
fn unclosed_element() {
    let src = "<Root><Child></Root>";
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn mismatched_closing_tag() {
    let src = "<a><b></c></a>";
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn missing_element_name() {
    let src = "<>text</>";
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn attribute_without_value() {
    let src = r#"<Root a= b="x"/>"#;
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn attribute_without_equals() {
    let src = r#"<Root a "orphan" b="x"/>"#;
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn unterminated_attribute_string() {
    let src = r#"<Root a="oops>"#;
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn stray_closing_tag_at_root() {
    let src = "<a/></b>";
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn closing_tag_with_attributes() {
    let src = r#"<a></a foo="bar">"#;
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn stray_xml_declaration_mid_document() {
    let src = r#"<Root><?xml version="1.0"?></Root>"#;
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn junk_between_elements() {
    let src = "<Root> ]]> &&& <Child/></Root>";
    assert_lossless(src);
}

#[test]
fn unterminated_comment() {
    let src = "<Root><!-- never ends";
    let parsed = parse(src);
    assert!(parsed.has_errors());
    assert_lossless(src);
}

#[test]
fn deeply_nested_does_not_stack_overflow() {
    let depth = 500;
    let src = format!("{}{}", "<x>".repeat(depth), "</x>".repeat(depth));
    let parsed = parse(&src);
    assert_eq!(parsed.syntax().to_string(), src);
}
