//! Semantic model for Stack XML resources / dictionaries.
//!
//! For now this is just a document outline (`document_symbols`); hover,
//! go-to-definition and lint build on the same tree walk later.

use biome_rowan::{AstNode, AstNodeList, TextRange};
use xml_syntax::{
    AnyXmlAttribute, AnyXmlElement, XmlAttributeList, XmlElementList, XmlName, XmlRoot,
    XmlSyntaxNode, inner_string_text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// An element that contains other elements.
    Container,
    /// A leaf element (self-closing, or with only text / attributes).
    Leaf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSymbol {
    pub name: String,
    /// The tag name, shown alongside `name` when `name` came from an
    /// attribute (`Имя="..."`).
    pub detail: Option<String>,
    pub kind: SymbolKind,
    /// Full text range of the element.
    pub range: TextRange,
    /// Range of the element's name token.
    pub selection_range: TextRange,
    pub children: Vec<DocumentSymbol>,
}

/// Attribute names that carry an element's display label. `Имя` is the
/// Stack resource/dictionary convention; the rest cover generic XML.
const NAME_ATTRS: &[&str] = &["Имя", "name", "Name", "id"];

/// Build a nested outline of an XML document from its parsed root.
pub fn document_symbols(root: &XmlSyntaxNode) -> Vec<DocumentSymbol> {
    let Some(root) = XmlRoot::cast(root.clone()) else {
        return Vec::new();
    };
    symbols_from_list(&root.content())
}

fn symbols_from_list(list: &XmlElementList) -> Vec<DocumentSymbol> {
    list.iter().filter_map(symbol_from_element).collect()
}

fn symbol_from_element(element: AnyXmlElement) -> Option<DocumentSymbol> {
    match element {
        AnyXmlElement::XmlElement(el) => {
            let opening = el.opening().ok()?;
            let name_node = opening.name().ok()?;
            let (name, detail) = label(&name_text(&name_node), &opening.attributes());
            let children = symbols_from_list(&el.children());
            Some(DocumentSymbol {
                name,
                detail,
                kind: if children.is_empty() {
                    SymbolKind::Leaf
                } else {
                    SymbolKind::Container
                },
                range: el.syntax().text_trimmed_range(),
                selection_range: name_node.syntax().text_trimmed_range(),
                children,
            })
        }
        AnyXmlElement::XmlSelfClosingElement(el) => {
            let name_node = el.name().ok()?;
            let (name, detail) = label(&name_text(&name_node), &el.attributes());
            Some(DocumentSymbol {
                name,
                detail,
                kind: SymbolKind::Leaf,
                range: el.syntax().text_trimmed_range(),
                selection_range: name_node.syntax().text_trimmed_range(),
                children: Vec::new(),
            })
        }
        _ => None,
    }
}

fn name_text(name: &XmlName) -> String {
    name.value_token()
        .map(|t| t.text_trimmed().to_string())
        .unwrap_or_default()
}

/// Decide the symbol's display name and detail: a name-carrying attribute
/// (if present) becomes the name and the tag becomes the detail; otherwise
/// the tag is the name.
fn label(tag: &str, attributes: &XmlAttributeList) -> (String, Option<String>) {
    for attr in attributes.iter() {
        let AnyXmlAttribute::XmlAttribute(attr) = attr else {
            continue;
        };
        let Ok(attr_name) = attr.name() else { continue };
        if !NAME_ATTRS.contains(&name_text(&attr_name).as_str()) {
            continue;
        }

        if let Some(init) = attr.initializer()
            && let Ok(value) = init.value()
            && let Ok(token) = value.value_token()
        {
            let value = inner_string_text(&token);
            let value = value.text().trim();
            if !value.is_empty() {
                return (value.to_string(), Some(tag.to_string()));
            }
        }
    }

    (tag.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml_parser::parse;

    fn symbols(src: &str) -> Vec<DocumentSymbol> {
        document_symbols(&parse(src).syntax())
    }

    #[test]
    fn empty_document_has_no_symbols() {
        assert!(symbols("").is_empty());
    }

    #[test]
    fn nests_elements_and_uses_the_name_attribute() {
        let syms = symbols(
            r#"<?xml version="1.1"?>
<config>
   <group name="first">
      <item id="a"/>
      <item id="b"/>
   </group>
</config>"#,
        );

        assert_eq!(syms.len(), 1);
        let config = &syms[0];
        assert_eq!(config.name, "config");
        assert_eq!(config.kind, SymbolKind::Container);

        assert_eq!(config.children.len(), 1);
        let group = &config.children[0];
        assert_eq!(group.name, "first");
        assert_eq!(group.detail.as_deref(), Some("group"));
        assert_eq!(group.kind, SymbolKind::Container);

        assert_eq!(group.children.len(), 2);
        assert_eq!(group.children[0].name, "a");
        assert_eq!(group.children[0].kind, SymbolKind::Leaf);
        assert_eq!(group.children[1].name, "b");
    }

    #[test]
    fn falls_back_to_the_tag_name() {
        let syms = symbols("<root><child/></root>");
        assert_eq!(syms[0].name, "root");
        assert_eq!(syms[0].children[0].name, "child");
        assert_eq!(syms[0].children[0].detail, None);
    }

    #[test]
    fn ignores_comments_and_processing_instructions() {
        let syms = symbols("<root><!-- c --><?pi x?><a/></root>");
        assert_eq!(syms[0].children.len(), 1);
        assert_eq!(syms[0].children[0].name, "a");
    }

    #[test]
    fn selection_range_covers_the_name_token() {
        let src = "<root><child/></root>";
        let syms = symbols(src);
        let sel = syms[0].children[0].selection_range;
        assert_eq!(&src[sel], "child");
    }
}
