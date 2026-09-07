mod parse_error;

use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use xml_syntax::T;
use xml_syntax::XmlSyntaxKind::{self, *};

use crate::XmlParser;
use crate::lexer::XmlLexContext;
use crate::syntax::parse_error::*;

const ATTRIBUTE_LIST_RECOVERY: TokenSet<XmlSyntaxKind> =
    token_set![T![>], T![<], T![/], QUESTION_R_ANGLE];

pub(crate) fn parse_root(p: &mut XmlParser) {
    let m = p.start();

    p.eat(UNICODE_BOM);
    parse_prolog(p).ok();
    ElementList { parent: None }.parse_list(p);

    m.complete(p, XML_ROOT);
}

// --- prolog -------------------------------------------------------------

fn parse_prolog(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(XML_DECL_START) {
        return Absent;
    }

    let m = p.start();
    p.bump(XML_DECL_START);
    AttributeList.parse_list(p);
    p.expect_with_context(QUESTION_R_ANGLE, XmlLexContext::ElementList);

    Present(m.complete(p, XML_PROLOG))
}

// --- elements ---------------------------------------------------------

#[derive(Default)]
struct ElementList {
    /// The name of the enclosing element, or `None` for the document root.
    parent: Option<String>,
}

impl ParseNodeList for ElementList {
    type Kind = XmlSyntaxKind;
    type Parser<'source> = XmlParser<'source>;
    const LIST_KIND: Self::Kind = XML_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T![<!--] => parse_comment(p),
            T!["<![CDATA["] => parse_cdata_section(p),
            L_ANGLE_QUESTION => parse_processing_instruction(p),
            XML_DECL_START => parse_stray_prolog(p),
            T![<] if p.nth_at(1, T![/]) => Absent,
            T![<] => parse_xml_element(p),
            XML_LITERAL => parse_text(p),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at(EOF) {
            return true;
        }
        self.parent.is_some() && p.at(T![<]) && p.nth_at(1, T![/])
    }

    fn recover(&mut self, p: &mut Self::Parser<'_>, parsed: ParsedSyntax) -> RecoveryResult {
        parsed.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(XML_BOGUS_ELEMENT, token_set![T![<], EOF]),
            expected_child,
        )
    }
}

fn parse_xml_element(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);

    let name = p.cur_text().to_string();
    parse_name(p).or_add_diagnostic(p, expected_element_name);
    AttributeList.parse_list(p);

    if p.at(T![/]) {
        p.bump(T![/]);
        p.expect_with_context(T![>], XmlLexContext::ElementList);
        return Present(m.complete(p, XML_SELF_CLOSING_ELEMENT));
    }

    p.expect_with_context(T![>], XmlLexContext::ElementList);
    let opening = m.complete(p, XML_OPENING_ELEMENT);

    loop {
        ElementList {
            parent: Some(name.clone()),
        }
        .parse_list(p);

        if let Some(mut closing) =
            parse_closing_element(p).or_add_diagnostic(p, expected_closing_tag)
            && !closing_matches(p, &closing, &name)
        {
            p.error(expected_matching_closing_tag(p, closing.range(p)));
            closing.change_to_bogus(p);
            continue;
        }
        break;
    }

    Present(opening.precede(p).complete(p, XML_ELEMENT))
}

fn closing_matches(p: &XmlParser, closing: &CompletedMarker, opening_name: &str) -> bool {
    closing.text(p).contains(opening_name.trim())
}

fn parse_closing_element(p: &mut XmlParser) -> ParsedSyntax {
    if !(p.at(T![<]) && p.nth_at(1, T![/])) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    p.bump(T![/]);
    parse_name(p).or_add_diagnostic(p, expected_element_name);

    while p.at(XML_LITERAL) || p.at(T![=]) || p.at(XML_STRING_LITERAL) {
        p.error(closing_tag_should_not_have_attributes(p, p.cur_range()));
        p.bump_remap(XML_BOGUS);
    }

    p.expect_with_context(T![>], XmlLexContext::ElementList);
    Present(m.complete(p, XML_CLOSING_ELEMENT))
}

// --- attributes -----------------------------------------------------

#[derive(Default)]
struct AttributeList;

impl ParseNodeList for AttributeList {
    type Kind = XmlSyntaxKind;
    type Parser<'source> = XmlParser<'source>;
    const LIST_KIND: Self::Kind = XML_ATTRIBUTE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_attribute(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![>]) || p.at(T![/]) || p.at(QUESTION_R_ANGLE)
    }

    fn recover(&mut self, p: &mut Self::Parser<'_>, parsed: ParsedSyntax) -> RecoveryResult {
        parsed.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(XML_BOGUS_ATTRIBUTE, ATTRIBUTE_LIST_RECOVERY),
            expected_attribute,
        )
    }
}

fn parse_attribute(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(XML_LITERAL) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).or_add_diagnostic(p, expected_attribute);

    if p.at(T![=]) {
        let init = p.start();
        p.bump(T![=]);
        parse_string(p).or_add_diagnostic(p, expected_attribute_value);
        init.complete(p, XML_ATTRIBUTE_INITIALIZER_CLAUSE);
    }

    Present(m.complete(p, XML_ATTRIBUTE))
}

// --- comments / CDATA / processing instructions --------------------

fn parse_comment(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(T![<!--]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![<!--], XmlLexContext::Comment);
    p.eat_with_context(XML_LITERAL, XmlLexContext::Comment);
    p.expect_with_context(T![-->], XmlLexContext::ElementList);

    Present(m.complete(p, XML_COMMENT))
}

fn parse_cdata_section(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(T!["<![CDATA["]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!["<![CDATA["], XmlLexContext::Cdata);
    p.eat_with_context(XML_LITERAL, XmlLexContext::Cdata);
    p.expect_with_context(T!["]]>"], XmlLexContext::ElementList);

    Present(m.complete(p, XML_CDATA_SECTION))
}

fn parse_processing_instruction(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(L_ANGLE_QUESTION) {
        return Absent;
    }

    let m = p.start();
    p.bump(L_ANGLE_QUESTION);

    if p.at(XML_LITERAL) {
        let target = p.start();
        p.bump_with_context(XML_LITERAL, XmlLexContext::PiContent);
        target.complete(p, XML_NAME);
    } else {
        p.error(expected_pi_target(p, p.cur_range()));
    }

    p.eat_with_context(XML_LITERAL, XmlLexContext::PiContent);
    p.expect_with_context(QUESTION_R_ANGLE, XmlLexContext::ElementList);

    Present(m.complete(p, XML_PROCESSING_INSTRUCTION))
}

/// An `<?xml ... ?>` declaration that appears somewhere other than the very
/// start of the document. Parsed with the same shape as [parse_prolog] but
/// flagged and kept as a bogus element so the tree stays lossless.
fn parse_stray_prolog(p: &mut XmlParser) -> ParsedSyntax {
    let m = p.start();
    p.error(ParseDiagnostic::new(
        "An XML declaration is only allowed at the start of the document.",
        p.cur_range(),
    ));
    p.bump(XML_DECL_START);
    AttributeList.parse_list(p);
    p.expect_with_context(QUESTION_R_ANGLE, XmlLexContext::ElementList);

    Present(m.complete(p, XML_BOGUS_ELEMENT))
}

// --- leaves ---------------------------------------------------------

fn parse_name(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(XML_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(XML_LITERAL);
    Present(m.complete(p, XML_NAME))
}

fn parse_string(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(XML_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(XML_STRING_LITERAL);
    Present(m.complete(p, XML_STRING))
}

fn parse_text(p: &mut XmlParser) -> ParsedSyntax {
    if !p.at(XML_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(XML_LITERAL, XmlLexContext::ElementList);
    Present(m.complete(p, XML_TEXT))
}
