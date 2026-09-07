#![cfg(test)]
#![allow(unused_mut)]

use super::{XmlLexContext, XmlLexer};
use biome_parser::lexer::Lexer;
use biome_rowan::TextSize;
use xml_syntax::XmlSyntaxKind::{self, EOF};

/// Lex `$src` in `$context` and assert the exact token kinds and byte
/// lengths, then check the token stream is lossless (reconstructs `$src`).
macro_rules! assert_lex {
    ($context:expr, $src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = XmlLexer::from_str($src);
        let mut idx = 0;
        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token($context) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                XmlSyntaxKind::$kind,
                "expected token kind {} at index {}, but found {:?}",
                stringify!($kind),
                idx,
                tokens[idx].0,
            );
            assert_eq!(
                tokens[idx].1.len(),
                TextSize::from($len),
                "expected token length {} for {}, but found {:?}",
                $len,
                stringify!($kind),
                tokens[idx].1.len(),
            );
            new_str.push_str(&$src[tokens[idx].1]);
            idx += 1;
        )*

        assert_eq!(
            idx,
            tokens.len(),
            "expected {} tokens but lexer produced {} (next: {:?})",
            idx,
            tokens.len(),
            tokens.get(idx).map(|t| t.0),
        );
        assert_eq!($src, new_str, "token stream is not lossless");
    }};
}

#[test]
fn empty() {
    assert_lex! { XmlLexContext::Regular, "", }
    assert_lex! { XmlLexContext::ElementList, "", }
}

#[test]
fn opening_tag_with_attributes() {
    assert_lex! {
        XmlLexContext::Regular,
        r#"<Foo a="1" b="two">"#,
        L_ANGLE: 1,
        XML_LITERAL: 3,
        WHITESPACE: 1,
        XML_LITERAL: 1,
        EQ: 1,
        XML_STRING_LITERAL: 3,
        WHITESPACE: 1,
        XML_LITERAL: 1,
        EQ: 1,
        XML_STRING_LITERAL: 5,
        R_ANGLE: 1,
    }
}

#[test]
fn self_closing_and_closing_tags() {
    assert_lex! {
        XmlLexContext::Regular,
        "<Foo/>",
        L_ANGLE: 1, XML_LITERAL: 3, SLASH: 1, R_ANGLE: 1,
    }
    assert_lex! {
        XmlLexContext::Regular,
        "</Foo>",
        L_ANGLE: 1, SLASH: 1, XML_LITERAL: 3, R_ANGLE: 1,
    }
}

#[test]
fn single_quoted_attribute() {
    assert_lex! {
        XmlLexContext::Regular,
        "<a b='x'/>",
        L_ANGLE: 1, XML_LITERAL: 1, WHITESPACE: 1, XML_LITERAL: 1, EQ: 1,
        XML_STRING_LITERAL: 3, SLASH: 1, R_ANGLE: 1,
    }
}

#[test]
fn attribute_value_may_contain_angle_brackets_and_newlines() {
    assert_lex! {
        XmlLexContext::Regular,
        // The attribute value spans a literal newline and contains `>`/`<`.
        "<p e=\"a>0\nand b<9\"/>",
        L_ANGLE: 1, XML_LITERAL: 1, WHITESPACE: 1, XML_LITERAL: 1, EQ: 1,
        XML_STRING_LITERAL: 13, SLASH: 1, R_ANGLE: 1,
    }
}

#[test]
fn cyrillic_names_and_values() {
    // `тег` = 3 chars / 6 bytes, `имя` = 3 / 6, `"значение"` = 8 chars + 2
    // quotes = 18 bytes.
    assert_lex! {
        XmlLexContext::Regular,
        r#"<тег имя="значение"/>"#,
        L_ANGLE: 1,
        XML_LITERAL: 6,
        WHITESPACE: 1,
        XML_LITERAL: 6,
        EQ: 1,
        XML_STRING_LITERAL: 18,
        SLASH: 1,
        R_ANGLE: 1,
    }
}

#[test]
fn entity_references_are_kept_verbatim_in_values() {
    assert_lex! {
        XmlLexContext::Regular,
        r#"<a b="&apos;x&apos;"/>"#,
        L_ANGLE: 1, XML_LITERAL: 1, WHITESPACE: 1, XML_LITERAL: 1, EQ: 1,
        XML_STRING_LITERAL: 15, SLASH: 1, R_ANGLE: 1,
    }
}

#[test]
fn xml_declaration() {
    assert_lex! {
        XmlLexContext::Regular,
        r#"<?xml version="1.1" encoding="UTF-8"?>"#,
        XML_DECL_START: 5,
        WHITESPACE: 1,
        XML_LITERAL: 7,
        EQ: 1,
        XML_STRING_LITERAL: 5,
        WHITESPACE: 1,
        XML_LITERAL: 8,
        EQ: 1,
        XML_STRING_LITERAL: 7,
        QUESTION_R_ANGLE: 2,
    }
}

#[test]
fn xml_declaration_without_space_before_close() {
    assert_lex! {
        XmlLexContext::Regular,
        "<?xml?>",
        XML_DECL_START: 5, QUESTION_R_ANGLE: 2,
    }
}

#[test]
fn processing_instruction() {
    assert_lex! {
        XmlLexContext::Regular,
        "<?target",
        L_ANGLE_QUESTION: 2, XML_LITERAL: 6,
    }
    assert_lex! {
        XmlLexContext::PiContent,
        " some data ?>",
        XML_LITERAL: 11, QUESTION_R_ANGLE: 2,
    }
}

#[test]
fn comment() {
    assert_lex! {
        XmlLexContext::Comment,
        "<!-- a comment -->",
        COMMENT_START: 4, XML_LITERAL: 11, COMMENT_END: 3,
    }
    assert_lex! {
        XmlLexContext::Comment,
        "<!---->",
        COMMENT_START: 4, COMMENT_END: 3,
    }
}

#[test]
fn cdata_section() {
    assert_lex! {
        XmlLexContext::Cdata,
        "<![CDATA[a > b]]>",
        CDATA_START: 9, XML_LITERAL: 5, CDATA_END: 3,
    }
}

#[test]
fn element_content_text_and_trivia() {
    // The `ElementList` context handles whitespace/newline trivia, free
    // text, and the first token of a markup start; the parser switches
    // context after that first token.
    assert_lex! {
        XmlLexContext::ElementList,
        "\n   ",
        NEWLINE: 1,
        WHITESPACE: 3,
    }
    assert_lex! {
        XmlLexContext::ElementList,
        "hello world ",
        XML_LITERAL: 12,
    }
    assert_lex! {
        XmlLexContext::ElementList,
        "text<",
        XML_LITERAL: 4, L_ANGLE: 1,
    }
}

#[test]
fn element_list_recognizes_markup_starts() {
    assert_lex! { XmlLexContext::ElementList, "<!--", COMMENT_START: 4 }
    assert_lex! { XmlLexContext::ElementList, "<![CDATA[", CDATA_START: 9 }
    assert_lex! { XmlLexContext::ElementList, "<?xml", XML_DECL_START: 5 }
    assert_lex! { XmlLexContext::ElementList, "<?", L_ANGLE_QUESTION: 2 }
}

#[test]
fn bom_is_consumed_once_at_start() {
    assert_lex! {
        XmlLexContext::ElementList,
        "\u{feff}text",
        UNICODE_BOM: 3, XML_LITERAL: 4,
    }
}

#[test]
fn unterminated_string_is_an_error() {
    assert_lex! {
        XmlLexContext::Regular,
        r#"<a b="oops"#,
        L_ANGLE: 1, XML_LITERAL: 1, WHITESPACE: 1, XML_LITERAL: 1, EQ: 1,
        ERROR_TOKEN: 5,
    }
}
