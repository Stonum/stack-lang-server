#[macro_use]
mod helper;

#[test]
fn empty_document() {
    assert_fmt!("");
}

#[test]
fn declaration_only() {
    assert_fmt!("<?xml version=\"1.1\" encoding=\"UTF-8\"?>\n");
}

#[test]
fn canonical_document_round_trips() {
    assert_fmt!(
        r#"<?xml version="1.1" encoding="UTF-8"?>
<root>
  <group name="a">
    <item x="1"/>
    <item x="2"/>
  </group>
  <!-- a note -->
</root>
"#
    );
}

#[test]
fn normalizes_whitespace_and_indentation() {
    assert_fmt_eq!(
        r#"<?xml    version="1.1"   encoding="UTF-8"?>
<root>
      <group    name="a">
   <item x="1"/>
            <item x="2"/>
</group>
</root>"#,
        r#"<?xml version="1.1" encoding="UTF-8"?>
<root>
  <group name="a">
    <item x="1"/>
    <item x="2"/>
  </group>
</root>
"#
    );
}

#[test]
fn normalizes_self_closing_spacing_to_tight() {
    assert_fmt_eq!(
        r#"<root>
  <a x="1" />
  <b/>
</root>"#,
        r#"<root>
  <a x="1"/>
  <b/>
</root>
"#
    );
}

#[test]
fn empty_element_collapses_to_one_line() {
    assert_fmt_eq!(
        r#"<root>
  <a>
  </a>
  <b></b>
</root>"#,
        r#"<root>
  <a></a>
  <b></b>
</root>
"#
    );
}

#[test]
fn long_self_closing_attribute_list_wraps_and_moves_the_close() {
    assert_fmt_eq!(
        r#"<node alpha="1" bravo="2" charlie="3" delta="4" echo="5" foxtrot="6" golf="7" hotel="8" india="9" juliet="10" kilo="11" lima="12" mike="13" november="14"/>"#,
        r#"<node
  alpha="1"
  bravo="2"
  charlie="3"
  delta="4"
  echo="5"
  foxtrot="6"
  golf="7"
  hotel="8"
  india="9"
  juliet="10"
  kilo="11"
  lima="12"
  mike="13"
  november="14"
/>
"#
    );
}

#[test]
fn long_opening_tag_wraps_and_moves_the_close() {
    assert_fmt_eq!(
        r#"<node alpha="1" bravo="2" charlie="3" delta="4" echo="5" foxtrot="6" golf="7" hotel="8" india="9" juliet="10" kilo="11" lima="12" mike="13" november="14">
  <child/>
</node>"#,
        r#"<node
  alpha="1"
  bravo="2"
  charlie="3"
  delta="4"
  echo="5"
  foxtrot="6"
  golf="7"
  hotel="8"
  india="9"
  juliet="10"
  kilo="11"
  lima="12"
  mike="13"
  november="14"
>
  <child/>
</node>
"#
    );
}

#[test]
fn attribute_values_are_verbatim() {
    // Quote style, entity references and embedded `>`/`<` must survive.
    assert_fmt!(
        r#"<field expr="case when (a.x>0 and a.y=5) then &apos;p&apos; else &apos;&apos; end" alt='single'/>
"#
    );
}

#[test]
fn multiline_attribute_value_is_preserved() {
    assert_fmt!(
        r#"<query join="
   line one
   line two
"/>
"#
    );
}

#[test]
fn comment_and_cdata_content_verbatim() {
    assert_fmt!(
        r#"<root>
  <!--   spaced   comment   -->
  <node><![CDATA[ raw <text> & more ]]></node>
</root>
"#
    );
}

#[test]
fn processing_instruction() {
    assert_fmt!(
        r#"<?xml version="1.0"?>
<?target some data?>
<root/>
"#
    );
}

#[test]
fn leading_bom_is_preserved() {
    assert_fmt!(concat!(
        "\u{feff}",
        r#"<?xml version="1.0"?>
<root/>
"#
    ));
}

#[test]
fn document_without_prolog() {
    assert_fmt_eq!(
        "<root><child/></root>",
        r#"<root>
  <child/>
</root>
"#
    );
}

#[test]
fn resource_defaults_to_three_space_indent() {
    assert_fmt_eq!(
        "<root><a><b/></a></root>",
        "<root>\n   <a>\n      <b/>\n   </a>\n</root>\n",
        xml_syntax::XmlFileSource::resource(),
    );
}

#[test]
fn dictionary_defaults_to_four_space_indent() {
    assert_fmt_eq!(
        "<root><a><b/></a></root>",
        "<root>\n    <a>\n        <b/>\n    </a>\n</root>\n",
        xml_syntax::XmlFileSource::dictionary(),
    );
}

#[test]
fn stray_text_between_elements_is_trimmed() {
    // The lexer folds trailing indentation into a text token; the formatter
    // must trim it so the result is idempotent. (Real resources occasionally
    // carry a stray character between elements.)
    assert_fmt_eq!(
        "<list>\n  <item x=\"1\"/>\n  x\n  <item x=\"2\"/>\n</list>",
        r#"<list>
  <item x="1"/>
  x
  <item x="2"/>
</list>
"#
    );
}
