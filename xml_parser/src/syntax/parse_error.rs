use crate::XmlParser;
use biome_parser::diagnostic::{expect_one_of, expected_node};
use biome_parser::prelude::*;
use biome_rowan::TextRange;

pub(crate) fn expected_element_name(p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("element name", range, p)
}

pub(crate) fn expected_pi_target(p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("processing instruction target", range, p)
}

pub(crate) fn expected_attribute(p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("attribute", range, p)
}

pub(crate) fn expected_attribute_value(p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("attribute value", range, p)
}

pub(crate) fn expected_child(p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["element", "text"], range).into_diagnostic(p)
}

pub(crate) fn expected_closing_tag(p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("closing tag", range, p)
}

pub(crate) fn expected_matching_closing_tag(_p: &XmlParser, range: TextRange) -> ParseDiagnostic {
    ParseDiagnostic::new("Expected a closing tag matching the opening tag.", range)
}

pub(crate) fn closing_tag_should_not_have_attributes(
    _p: &XmlParser,
    range: TextRange,
) -> ParseDiagnostic {
    ParseDiagnostic::new("Closing tags should not have attributes.", range)
}
