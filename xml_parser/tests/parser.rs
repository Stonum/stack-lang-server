#[macro_use]
mod helper;

use xml_parser::parse;

#[test]
fn empty_input() {
    let res = parse("");
    assert_parser!(res);
}

#[test]
fn declaration_only() {
    let res = parse(r#"<?xml version="1.1" encoding="UTF-8"?>"#);
    assert_parser!(res);
}

#[test]
fn minimal_document() {
    let res = parse(
        r#"<?xml version="1.1" encoding="UTF-8"?>
<root>
</root>"#,
    );
    assert_parser!(res);
}

#[test]
fn self_closing_children_with_attributes() {
    let res = parse(
        r#"<?xml version="1.1" encoding="UTF-8"?>
<root>
   <group name="first" kind="list">
      <item text="one"/>
      <item text="two" />
   </group>
</root>"#,
    );
    assert_parser!(res);
}

#[test]
fn single_and_double_quoted_attributes() {
    let res = parse(r#"<root a="x" b='y'/>"#);
    assert_parser!(res);
}

#[test]
fn attribute_value_with_angle_brackets_and_entities() {
    let res = parse(
        r#"<field name="flag" expr="case when (a.x>0 and a.y=5) then &apos;p&apos; else &apos;&apos; end"/>"#,
    );
    assert_parser!(res);
}

#[test]
fn multiline_attribute_value() {
    let res = parse(
        r#"<query join="
   line one
   line two
"/>"#,
    );
    assert_parser!(res);
}

#[test]
fn cyrillic_names_and_values() {
    let res = parse(r#"<раздел имя="первый"><запись текст="привет"/></раздел>"#);
    assert_parser!(res);
}

#[test]
fn comment_and_cdata() {
    let res = parse(
        r#"<root>
   <!-- a comment -->
   <node><![CDATA[ raw <text> & stuff ]]></node>
</root>"#,
    );
    assert_parser!(res);
}

#[test]
fn empty_comment_and_empty_element() {
    let res = parse("<root><!----><child/></root>");
    assert_parser!(res);
}

#[test]
fn deeply_nested_shape() {
    let res = parse(
        r#"<?xml version="1.1" encoding="UTF-8"?>
<root>
   <outer>
      <inner name="a" note="b">
         <fields>
            <field name="d1" type="date"/>
            <field name="t1" type="string" size="8000"/>
         </fields>
         <indexes>
            <index name="status" fields="status"/>
         </indexes>
         <triggers/>
      </inner>
   </outer>
</root>"#,
    );
    assert_parser!(res);
}

#[test]
fn processing_instruction_is_accepted() {
    let res = parse(r#"<?xml version="1.0"?><?target some data?><root/>"#);
    assert_parser!(res);
}

#[test]
fn leading_bom() {
    let res = parse(concat!("\u{feff}", r#"<?xml version="1.0"?><root/>"#));
    assert_parser!(res);
}

#[test]
fn round_trips_losslessly() {
    let sources = [
        "",
        "<root/>",
        r#"<?xml version="1.1"?>
<config>
   <a x="1"/>
</config>"#,
        "<root>text<child/>more</root>",
    ];

    for src in sources {
        let parsed = parse(src);
        assert_eq!(
            parsed.syntax().to_string(),
            src,
            "parser is not lossless for {src:?}"
        );
    }
}
