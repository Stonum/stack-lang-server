use std::collections::HashMap;

use biome_rowan::{AstNode, AstNodeList, TextRange};
use xml_syntax::{
    AnyXmlElement, XmlAttributeList, XmlElementList, XmlSyntaxNode, inner_string_text,
};

use crate::{Diagnostic, Severity};

pub const CODE: &str = "duplicate-named-sibling";

/// Flags sibling elements that share both their tag name and the value of
/// their name attribute — e.g. two `<Поле Имя="X">` inside the same parent, or
/// two `<Select Имя="X">` inside a `<Resource>`. That collision is almost
/// always a copy-paste slip.
///
/// The name attribute is matched case-insensitively against `Имя` / `name`;
/// tag names and the name value itself are compared case-sensitively (`Имя` is
/// a distinct value from `имя`), matching `duplicate-attribute`.
pub fn check(root: &XmlSyntaxNode) -> Vec<Diagnostic> {
    root.descendants()
        .filter_map(XmlElementList::cast)
        .flat_map(|list| check_scope(&list))
        .collect()
}

/// A `(tag name, name value)` pair — two elements with the same key in one
/// scope are the duplicates we report.
type Key = (String, String);

struct Named {
    key: Key,
    range: TextRange,
}

fn check_scope(list: &XmlElementList) -> Vec<Diagnostic> {
    let named: Vec<Named> = list.iter().filter_map(|el| named_element(&el)).collect();

    let mut counts: HashMap<&Key, usize> = HashMap::new();
    for item in &named {
        *counts.entry(&item.key).or_default() += 1;
    }

    // Every member of a colliding group is flagged (not just the extras): the
    // siblings are equally suspect and the reader wants to see all of them.
    named
        .iter()
        .filter(|item| counts.get(&item.key).is_some_and(|&n| n >= 2))
        .map(|item| {
            let (tag, value) = &item.key;
            Diagnostic {
                severity: Severity::Error,
                code: CODE,
                message: format!("Duplicate <{tag}> with the same name `{value}` in this scope."),
                range: item.range,
            }
        })
        .collect()
}

fn named_element(element: &AnyXmlElement) -> Option<Named> {
    let (name, attributes) = match element {
        AnyXmlElement::XmlElement(el) => {
            let opening = el.opening().ok()?;
            (opening.name().ok()?, opening.attributes())
        }
        AnyXmlElement::XmlSelfClosingElement(el) => (el.name().ok()?, el.attributes()),
        _ => return None,
    };

    let tag = name.value_token().ok()?.text_trimmed().to_string();
    let value = name_attribute_value(&attributes)?;

    Some(Named {
        key: (tag, value),
        range: name.syntax().text_trimmed_range(),
    })
}

fn name_attribute_value(attributes: &XmlAttributeList) -> Option<String> {
    attributes.iter().find_map(|attribute| {
        let attribute = attribute.as_xml_attribute()?;
        let key = attribute.name().ok()?.value_token().ok()?;
        let key = key.text_trimmed().to_lowercase();
        if key != "имя" && key != "name" {
            return None;
        }
        let token = attribute.initializer()?.value().ok()?.value_token().ok()?;
        Some(inner_string_text(&token).text().to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml_parser::parse;

    fn lint(src: &str) -> Vec<Diagnostic> {
        check(&parse(src).syntax())
    }

    #[test]
    fn flags_two_fields_with_the_same_name() {
        let diagnostics = lint(
            r#"<Поле>
   <Поле Имя="A"/>
   <Поле Имя="B"/>
   <Поле Имя="B"/>
</Поле>"#,
        );
        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].code, CODE);
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert!(diagnostics[0].message.contains("Поле"));
        assert!(diagnostics[0].message.contains('B'));
    }

    #[test]
    fn flags_every_member_of_the_group() {
        let diagnostics = lint(r#"<r><s Имя="x"/><s Имя="x"/><s Имя="x"/></r>"#);
        assert_eq!(diagnostics.len(), 3);
    }

    #[test]
    fn points_at_each_tag_name() {
        let src = r#"<r><Select Имя="x"/><Select Имя="x"/></r>"#;
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 2);
        for d in &diagnostics {
            assert_eq!(&src[d.range], "Select");
        }
    }

    #[test]
    fn different_tags_with_the_same_name_are_fine() {
        assert!(lint(r#"<r><Поле Имя="x"/><Колонка Имя="x"/></r>"#).is_empty());
    }

    #[test]
    fn different_names_are_fine() {
        assert!(lint(r#"<r><Поле Имя="x"/><Поле Имя="y"/></r>"#).is_empty());
    }

    #[test]
    fn elements_without_a_name_attribute_are_ignored() {
        assert!(lint(r#"<r><Поле/><Поле/></r>"#).is_empty());
    }

    #[test]
    fn each_scope_is_independent() {
        // `Поле Имя="x"` appears once per parent — no collision.
        assert!(
            lint(
                r#"<r>
   <g><Поле Имя="x"/></g>
   <g><Поле Имя="x"/></g>
</r>"#
            )
            .is_empty()
        );
    }

    #[test]
    fn nested_same_name_is_flagged_only_where_it_collides() {
        let diagnostics = lint(
            r#"<r>
   <Поле Имя="x">
      <Поле Имя="x"/>
      <Поле Имя="x"/>
   </Поле>
</r>"#,
        );
        // The outer `<Поле Имя="x">` is the lone child of `<r>`; only the two
        // inner ones collide.
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn the_name_attribute_lookup_is_case_insensitive() {
        let diagnostics = lint(r#"<r><s ИМЯ="x"/><s имя="x"/></r>"#);
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn the_name_value_is_case_sensitive() {
        assert!(lint(r#"<r><s Имя="x"/><s Имя="X"/></r>"#).is_empty());
    }

    #[test]
    fn mixes_self_closing_and_paired_tags() {
        let diagnostics = lint(r#"<r><s Имя="x"/><s Имя="x"></s></r>"#);
        assert_eq!(diagnostics.len(), 2);
    }
}
