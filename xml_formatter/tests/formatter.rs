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

/// Format `$src` with `verbatim_attributes` enabled and assert the result
/// equals `$dest`, then assert reformatting `$dest` is a no-op.
macro_rules! assert_verbatim_attrs_fmt_eq {
    ($src:expr, $dest:expr $(,)?) => {{
        use biome_formatter::{IndentStyle, LineWidth};
        use xml_formatter::{XmlFormatOptions, format_node};
        use xml_parser::parse;

        let options = || {
            XmlFormatOptions::new(xml_syntax::XmlFileSource::plain())
                .with_indent_style(IndentStyle::Space)
                .with_line_width(LineWidth::try_from(120).unwrap())
                .with_verbatim_attributes(true)
        };

        let run = |src: &str| {
            let tree = parse(src);
            assert!(!tree.has_errors(), "parse errors: {:?}", tree.diagnostics());
            format_node(options(), &tree.syntax())
                .unwrap()
                .print()
                .unwrap()
                .into_code()
        };

        let result = run($src);
        assert_eq!($dest, result, "input:\n{}\nformatted:\n{}", $src, result);
        assert_eq!(result, run(&result), "not idempotent:\n{}", result);
    }};
}

#[test]
fn verbatim_attributes_keeps_spacing_and_never_wraps() {
    // Only the tag's own indentation is normalized; the attribute region --
    // including the padding after the tag name -- comes through byte-for-byte,
    // and a long list is not wrapped.
    assert_verbatim_attrs_fmt_eq!(
        r#"<root><node   alpha="1"    bravo="2" charlie="3" delta="4" echo="5" foxtrot="6" golf="7" hotel="8" india="9" juliet="10" kilo="11" lima="12"/></root>"#,
        r#"<root>
  <node   alpha="1"    bravo="2" charlie="3" delta="4" echo="5" foxtrot="6" golf="7" hotel="8" india="9" juliet="10" kilo="11" lima="12"/>
</root>
"#
    );
}

#[test]
fn verbatim_attributes_preserve_one_attribute_per_line_layout() {
    // The hand-authored layout -- first attribute on its own line, each
    // attribute on its own line -- survives verbatim, and because it is
    // multi-line `/>` drops to a line of its own.
    assert_verbatim_attrs_fmt_eq!(
        "<Browser\n   Name=\"x\"\n   Kind=\"y\"\n   Expr=\"f(`a`,`b`)\"\n/>\n",
        "<Browser\n   Name=\"x\"\n   Kind=\"y\"\n   Expr=\"f(`a`,`b`)\"\n/>\n"
    );
}

#[test]
fn verbatim_attributes_keep_a_single_line_tag_on_one_line() {
    // Attributes authored on the tag's line stay there, `/>` included.
    assert_verbatim_attrs_fmt_eq!(
        "<root><a x=\"1\" y=\"2\"/></root>",
        "<root>\n  <a x=\"1\" y=\"2\"/>\n</root>\n"
    );
}

#[test]
fn verbatim_attributes_multiline_opening_tag_drops_its_close() {
    // A multi-line attribute region pushes `>` onto its own line, indented
    // with the tag.
    assert_verbatim_attrs_fmt_eq!(
        "<root><a x=\"1\"\n         y=\"2\"></a></root>",
        "<root>\n  <a x=\"1\"\n         y=\"2\"\n  ></a>\n</root>\n"
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
