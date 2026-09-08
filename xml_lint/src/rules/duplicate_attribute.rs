use std::collections::HashSet;

use biome_rowan::{AstNode, AstNodeList};
use xml_syntax::{AnyXmlAttribute, XmlAttributeList, XmlSyntaxNode};

use crate::{Diagnostic, Severity};

pub const CODE: &str = "duplicate-attribute";

/// XML forbids an element from carrying the same attribute name twice — it's
/// a well-formedness violation. The parser is deliberately tolerant, so this
/// is caught here instead. Attribute names are case-sensitive, so `Имя` and
/// `имя` are distinct.
pub fn check(root: &XmlSyntaxNode) -> Vec<Diagnostic> {
    root.descendants()
        .filter_map(XmlAttributeList::cast)
        .flat_map(|list| check_list(&list))
        .collect()
}

fn check_list(list: &XmlAttributeList) -> Vec<Diagnostic> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut diagnostics = Vec::new();

    for attribute in list.iter() {
        let AnyXmlAttribute::XmlAttribute(attribute) = attribute else {
            continue;
        };
        let Ok(name) = attribute.name() else { continue };
        let Ok(token) = name.value_token() else {
            continue;
        };
        let text = token.text_trimmed();

        if !seen.insert(text.to_string()) {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: CODE,
                message: format!("Duplicate attribute `{text}`."),
                range: name.syntax().text_trimmed_range(),
            });
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml_parser::parse;

    fn lint(src: &str) -> Vec<Diagnostic> {
        check(&parse(src).syntax())
    }

    #[test]
    fn flags_a_repeated_attribute() {
        let diagnostics = lint(r#"<a x="1" y="2" x="3"/>"#);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, CODE);
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert!(diagnostics[0].message.contains('x'));
    }

    #[test]
    fn points_at_the_later_occurrence() {
        let src = r#"<a x="1" x="3"/>"#;
        let diagnostics = lint(src);
        assert_eq!(&src[diagnostics[0].range], "x");
        // The offset is that of the *second* `x`, not the first.
        assert!(usize::from(diagnostics[0].range.start()) > 5);
    }

    #[test]
    fn flags_every_extra_occurrence() {
        let diagnostics = lint(r#"<a x="1" x="2" x="3"/>"#);
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn attribute_names_are_case_sensitive() {
        let diagnostics = lint(r#"<Поле Имя="a" имя="b"/>"#);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn distinct_attributes_are_fine() {
        assert!(lint(r#"<a x="1" y="2" z="3"/>"#).is_empty());
    }

    #[test]
    fn checks_opening_tags_and_nested_elements() {
        let diagnostics = lint(
            r#"<root a="1" a="2">
   <child b="1" b="2"/>
</root>"#,
        );
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn each_element_has_its_own_namespace() {
        // `x` on two different elements is not a duplicate.
        assert!(lint(r#"<root x="1"><child x="2"/></root>"#).is_empty());
    }
}
