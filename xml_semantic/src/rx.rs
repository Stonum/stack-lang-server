//! Semantic model of `.rx` resources: selects and API browsers linked to `.hdl` handlers.

use std::collections::HashMap;

use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize};
use line_index::{LineColRange, LineIndex};
use lsp_definition::{
    CodeSymbolDefinition, CodeSymbolInformation, DefinitionKind, LocationDefinition,
    MarkupDefinition, SemanticInfo, Symbol, SymbolKind, Usage, api_handler_name,
};
use xml_syntax::{
    AnyXmlAttribute, XmlAttribute, XmlAttributeList, XmlName, XmlOpeningElement,
    XmlSelfClosingElement, XmlSyntaxKind, XmlSyntaxNode, XmlSyntaxToken,
};

const SELECT_TAG: &str = "Select";
const API_BROWSER_TAG: &str = "APIBrowser";

const NAME_ATTR: &str = "Имя";
const SELECT_ATTR: &str = "Имя_выборки";
const HANDLER_ATTR: &str = "Обработчик";
const METHODS_ATTR: &str = "ДопМетоды";
const EXPRESSION_ATTR: &str = "Выражение";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RxName {
    pub text: String,
    pub range: LineColRange,
}

/// `<Select>` or `<APIBrowser>` declaration.
#[derive(Debug, PartialEq, Eq)]
pub struct RxDefinition {
    pub kind: DefinitionKind,
    pub id: RxName,
    /// `Имя_выборки` of an API browser.
    pub select: Option<RxName>,
    /// Extra built-in handler: `Обработчик`.
    pub handler: Option<RxName>,
    /// `ДопМетоды` of an API browser.
    pub methods: Vec<RxName>,
    pub range: LineColRange,
}

#[derive(Debug, Default)]
pub struct RxSemanticModel {
    definitions: Vec<RxDefinition>,
    references: HashMap<SemanticInfo, Vec<RxName>>,
}

impl RxSemanticModel {
    pub fn definitions(&self) -> core::slice::Iter<'_, RxDefinition> {
        self.definitions.iter()
    }

    /// Names used in `Имя_выборки`, `Обработчик` and `ДопМетоды`.
    pub fn references(&self) -> &HashMap<SemanticInfo, Vec<RxName>> {
        &self.references
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RxLinkKind {
    /// `Имя_выборки` of an API browser.
    Select,
    /// A handler of the browser's select.
    SelectHandler,
    /// A handler of the resource itself.
    Handler,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RxLink {
    pub kind: RxLinkKind,
    pub symbol: Symbol,
}

impl RxDefinition {
    /// What this resource is tied to by its own attributes: a browser to its select and
    /// API handler, a select to its handler, both to the extra `Обработчик`.
    pub fn links(&self) -> Vec<RxLink> {
        let link = |kind, symbol| RxLink { kind, symbol };
        let mut links = vec![];
        if let Some(select) = &self.select {
            links.push(link(
                RxLinkKind::Select,
                Symbol::Select(select.text.clone()),
            ));
        }
        let handler = match self.kind {
            DefinitionKind::ApiBrowser => api_handler_name(&self.id.text),
            _ => self.id.text.clone(),
        };
        links.push(link(RxLinkKind::Handler, Symbol::Handler(handler)));
        if let Some(handler) = &self.handler {
            links.push(link(
                RxLinkKind::Handler,
                Symbol::ExtraHandler(handler.text.clone()),
            ));
        }
        links
    }
}

pub fn rx_semantics(index: &LineIndex, root: &XmlSyntaxNode) -> RxSemanticModel {
    let definitions: Vec<_> = root
        .descendants()
        .filter_map(|node| rx_definition(index, &node))
        .collect();

    let mut references: HashMap<SemanticInfo, Vec<RxName>> = HashMap::new();
    let mut add = |symbol, name: &RxName| {
        let info = SemanticInfo::new(symbol, Usage::Reference);
        references.entry(info).or_default().push(name.clone());
    };
    for definition in &definitions {
        if let Some(select) = &definition.select {
            add(Symbol::Select(select.text.clone()), select);
        }
        if let Some(handler) = &definition.handler {
            add(Symbol::ExtraHandler(handler.text.clone()), handler);
        }
        for method in &definition.methods {
            let symbol = Symbol::HandlerEvent {
                handler: api_handler_name(&definition.id.text),
                event: method.text.clone(),
            };
            add(symbol, method);
        }
    }

    RxSemanticModel {
        definitions,
        references,
    }
}

fn rx_definition(index: &LineIndex, node: &XmlSyntaxNode) -> Option<RxDefinition> {
    let (tag, attributes) = tag_and_attributes(node)?;
    let kind = match tag.as_str() {
        SELECT_TAG => DefinitionKind::Select,
        API_BROWSER_TAG => DefinitionKind::ApiBrowser,
        _ => return None,
    };

    // an opening tag stands for its whole element
    let element = match XmlOpeningElement::can_cast(node.kind()) {
        true => node.parent()?,
        false => node.clone(),
    };

    let value = |name| {
        let token = attribute_value(&attributes, name)?;
        let (text, range) = string_value(&token);
        Some(RxName {
            text: text.to_string(),
            range: index.line_col_range(range)?,
        })
    };

    let methods = attribute_value(&attributes, METHODS_ATTR)
        .map(|token| {
            let (text, range) = string_value(&token);
            words(text, range.start())
                .filter_map(|(word, range)| {
                    Some(RxName {
                        text: word.to_string(),
                        range: index.line_col_range(range)?,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Some(RxDefinition {
        kind,
        id: value(NAME_ATTR)?,
        select: value(SELECT_ATTR),
        handler: value(HANDLER_ATTR),
        methods,
        range: index.line_col_range(element.text_trimmed_range())?,
    })
}

/// What an rx attribute value under the cursor refers to.
pub fn rx_identifier_for_offset(root: &XmlSyntaxNode, offset: TextSize) -> Option<SemanticInfo> {
    let token = root
        .token_at_offset(offset)
        .find(|t| t.kind() == XmlSyntaxKind::XML_STRING_LITERAL)?;
    let attribute = token.ancestors().find_map(XmlAttribute::cast)?;
    let (tag, attributes) = attribute
        .syntax()
        .ancestors()
        .find_map(|n| tag_and_attributes(&n))?;

    let (value, range) = string_value(&token);
    let value = value.to_string();
    let attribute = name_text(&attribute.name().ok()?);

    let (symbol, usage) = match (tag.as_str(), attribute.as_str()) {
        (SELECT_TAG, NAME_ATTR) => (Symbol::Select(value), Usage::Declaration),
        (API_BROWSER_TAG, NAME_ATTR) => (Symbol::ApiBrowser(value), Usage::Declaration),
        (API_BROWSER_TAG, SELECT_ATTR) => (Symbol::Select(value), Usage::Reference),
        (SELECT_TAG | API_BROWSER_TAG, HANDLER_ATTR) => {
            (Symbol::ExtraHandler(value), Usage::Reference)
        }
        (API_BROWSER_TAG, METHODS_ATTR) => {
            let (event, _) =
                words(&value, range.start()).find(|(_, range)| range.contains_inclusive(offset))?;
            let browser = attribute_value(&attributes, NAME_ATTR)?;
            let symbol = Symbol::HandlerEvent {
                handler: api_handler_name(string_value(&browser).0),
                event: event.to_string(),
            };
            (symbol, Usage::Reference)
        }
        _ => return None,
    };

    Some(SemanticInfo::new(symbol, usage))
}

/// mlang code of an `Выражение` attribute, with the cursor offset inside it.
#[derive(Debug, PartialEq, Eq)]
pub struct RxExpression {
    pub code: String,
    pub offset: TextSize,
}

/// The `Выражение` of an API browser under the cursor.
pub fn rx_expression_at(root: &XmlSyntaxNode, offset: TextSize) -> Option<RxExpression> {
    let token = root
        .token_at_offset(offset)
        .find(|t| t.kind() == XmlSyntaxKind::XML_STRING_LITERAL)?;
    let attribute = token.ancestors().find_map(XmlAttribute::cast)?;
    if name_text(&attribute.name().ok()?) != EXPRESSION_ATTR {
        return None;
    }
    let (tag, _) = attribute
        .syntax()
        .ancestors()
        .find_map(|n| tag_and_attributes(&n))?;
    if tag != API_BROWSER_TAG {
        return None;
    }

    let (value, range) = string_value(&token);
    if !range.contains_inclusive(offset) {
        return None;
    }
    let (code, cursor) = decode_entities(value, usize::from(offset - range.start()));
    Some(RxExpression {
        code,
        offset: TextSize::from(cursor as u32),
    })
}

/// Decodes the predefined XML entities, moving the `cursor` byte offset along.
fn decode_entities(text: &str, cursor: usize) -> (String, usize) {
    const ENTITIES: [(&str, char); 5] = [
        ("&quot;", '"'),
        ("&apos;", '\''),
        ("&amp;", '&'),
        ("&lt;", '<'),
        ("&gt;", '>'),
    ];

    let mut decoded = String::with_capacity(text.len());
    let mut decoded_cursor = None;
    let mut i = 0;
    while i < text.len() {
        if decoded_cursor.is_none() && i >= cursor {
            decoded_cursor = Some(decoded.len());
        }
        let rest = &text[i..];
        match ENTITIES.iter().find(|(entity, _)| rest.starts_with(entity)) {
            Some((entity, c)) => {
                decoded.push(*c);
                i += entity.len();
            }
            None => {
                let c = rest.chars().next().unwrap_or_default();
                decoded.push(c);
                i += c.len_utf8();
            }
        }
    }
    let cursor = decoded_cursor.unwrap_or(decoded.len());
    (decoded, cursor)
}

fn tag_and_attributes(node: &XmlSyntaxNode) -> Option<(String, XmlAttributeList)> {
    if let Some(opening) = XmlOpeningElement::cast(node.clone()) {
        return Some((name_text(&opening.name().ok()?), opening.attributes()));
    }
    let element = XmlSelfClosingElement::cast(node.clone())?;
    Some((name_text(&element.name().ok()?), element.attributes()))
}

fn attribute_value(attributes: &XmlAttributeList, name: &str) -> Option<XmlSyntaxToken> {
    attributes.iter().find_map(|attribute| {
        let AnyXmlAttribute::XmlAttribute(attribute) = attribute else {
            return None;
        };
        if name_text(&attribute.name().ok()?) != name {
            return None;
        }
        attribute.initializer()?.value().ok()?.value_token().ok()
    })
}

/// Unquoted text of an attribute value token and its range.
fn string_value(token: &XmlSyntaxToken) -> (&str, TextRange) {
    let range = token.text_trimmed_range();
    let text = token.text_trimmed();
    if text.len() < 2 {
        return (text, range);
    }
    let quote = TextSize::from(1);
    let inner = TextRange::new(range.start() + quote, range.end() - quote);
    (&text[1..text.len() - 1], inner)
}

fn words(text: &str, start: TextSize) -> impl Iterator<Item = (&str, TextRange)> {
    text.split_whitespace().map(move |word| {
        let offset = TextSize::from((word.as_ptr() as usize - text.as_ptr() as usize) as u32);
        let range = TextRange::at(start + offset, TextSize::of(word));
        (word, range)
    })
}

fn name_text(name: &XmlName) -> String {
    name.value_token()
        .map(|t| t.text_trimmed().to_string())
        .unwrap_or_default()
}

impl CodeSymbolDefinition for RxDefinition {
    fn kind(&self) -> DefinitionKind {
        self.kind
    }
    fn id(&self) -> &str {
        &self.id.text
    }
    fn parameters(&self) -> Option<&str> {
        None
    }
    fn container(&self) -> Option<Self> {
        None
    }
    fn parent(&self) -> Option<&str> {
        None
    }
    fn can_be_called(&self, _count: usize) -> bool {
        true
    }
    fn call_priority(&self, _another: &Self, _count: usize) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl LocationDefinition for RxName {
    fn range(&self) -> LineColRange {
        self.range
    }
}

impl LocationDefinition for RxDefinition {
    fn range(&self) -> LineColRange {
        self.range
    }
    fn id_range(&self) -> LineColRange {
        self.id.range
    }
}

impl CodeSymbolInformation for RxDefinition {
    fn symbol_kind(&self) -> SymbolKind {
        match self.kind {
            DefinitionKind::ApiBrowser => SymbolKind::INTERFACE,
            _ => SymbolKind::STRUCT,
        }
    }
}

impl MarkupDefinition for RxDefinition {
    fn markdown(&self) -> String {
        let tag = match self.kind {
            DefinitionKind::ApiBrowser => API_BROWSER_TAG,
            _ => SELECT_TAG,
        };
        let select = self
            .select
            .as_ref()
            .map(|s| format!(" {SELECT_ATTR}=\"{}\"", s.text))
            .unwrap_or_default();
        format!(
            "```xml\n<{tag} {NAME_ATTR}=\"{}\"{select}>\n```",
            self.id.text
        )
    }

    fn documentation(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml_parser::parse;

    const SRC: &str = r#"<?xml version="1.1"?>
<Resources>
   <Select Имя="Записи" Обработчик="Внешний">
      <Поля>
         <Поле Имя="Код"/>
      </Поля>
   </Select>
   <APIBrowser Имя="Модуль.Записи" Имя_выборки="Записи" ДопМетоды=" ДействиеБ  ДействиеА"/>
   <Other Имя="Прочее"/>
</Resources>"#;

    fn model() -> RxSemanticModel {
        rx_semantics(&LineIndex::new(SRC), &parse(SRC).syntax())
    }

    /// Columns are counted in chars, not bytes.
    fn text(range: LineColRange) -> String {
        assert_eq!(range.start.line, range.end.line);
        let line = SRC.lines().nth(range.start.line as usize).unwrap();
        let (start, end) = (range.start.col as usize, range.end.col as usize);
        line.chars().skip(start).take(end - start).collect()
    }

    /// Info at `$` inside `pattern`; `pattern` without `$` must occur once in `SRC`.
    fn at(pattern: &str) -> Option<SemanticInfo> {
        let pos = pattern.find('$').expect("pattern needs a `$` cursor");
        let needle = pattern.replace('$', "");
        assert_eq!(
            SRC.matches(&needle).count(),
            1,
            "`{needle}` must occur once"
        );
        let offset = SRC.find(&needle).unwrap() + pos;
        rx_identifier_for_offset(&parse(SRC).syntax(), TextSize::from(offset as u32))
    }

    #[test]
    fn collects_selects_and_api_browsers() {
        let model = model();
        let defs: Vec<_> = model.definitions().collect();
        assert_eq!(defs.len(), 2);

        let select = defs[0];
        assert_eq!(select.kind, DefinitionKind::Select);
        assert_eq!(text(select.id.range), "Записи");
        assert_eq!(
            select.handler.as_ref().map(|h| h.text.as_str()),
            Some("Внешний")
        );
        assert_eq!((select.range.start.line, select.range.end.line), (2, 6));

        let browser = defs[1];
        assert_eq!(browser.kind, DefinitionKind::ApiBrowser);
        assert_eq!(browser.id.text, "Модуль.Записи");
        assert_eq!(text(browser.select.as_ref().unwrap().range), "Записи");
        let methods: Vec<_> = browser.methods.iter().map(|m| text(m.range)).collect();
        assert_eq!(methods, ["ДействиеБ", "ДействиеА"]);
    }

    #[test]
    fn collects_attribute_references() {
        let model = model();
        let refs = model.references();
        let texts = |symbol| {
            let names = &refs[&SemanticInfo::new(symbol, Usage::Reference)];
            names.iter().map(|n| text(n.range)).collect::<Vec<_>>()
        };

        assert_eq!(refs.len(), 4);
        assert_eq!(texts(Symbol::Select("Записи".into())), ["Записи"]);
        assert_eq!(texts(Symbol::ExtraHandler("Внешний".into())), ["Внешний"]);
        let event = Symbol::HandlerEvent {
            handler: "Модуль.Записи_АПИ".into(),
            event: "ДействиеА".into(),
        };
        assert_eq!(texts(event), ["ДействиеА"]);
    }

    #[test]
    fn links_of_resources() {
        let model = model();
        let links: Vec<Vec<_>> = model
            .definitions()
            .map(|d| d.links().into_iter().map(|l| (l.kind, l.symbol)).collect())
            .collect();
        assert_eq!(
            links[0],
            [
                (RxLinkKind::Handler, Symbol::Handler("Записи".into())),
                (RxLinkKind::Handler, Symbol::ExtraHandler("Внешний".into()))
            ]
        );
        assert_eq!(
            links[1],
            [
                (RxLinkKind::Select, Symbol::Select("Записи".into())),
                (
                    RxLinkKind::Handler,
                    Symbol::Handler("Модуль.Записи_АПИ".into())
                )
            ]
        );
    }

    #[test]
    fn declarations_under_cursor() {
        assert_eq!(
            at(r#"Имя="З$аписи" Обработчик"#),
            Some(SemanticInfo::new(
                Symbol::Select("Записи".into()),
                Usage::Declaration
            ))
        );
        assert_eq!(
            at(r#"Имя="М$одуль.Записи""#),
            Some(SemanticInfo::new(
                Symbol::ApiBrowser("Модуль.Записи".into()),
                Usage::Declaration
            ))
        );
    }

    #[test]
    fn references_under_cursor() {
        assert_eq!(
            at(r#"Имя_выборки="$Записи""#),
            Some(SemanticInfo::new(
                Symbol::Select("Записи".into()),
                Usage::Reference
            ))
        );
        assert_eq!(
            at(r#"Обработчик="Внешний$""#),
            Some(SemanticInfo::new(
                Symbol::ExtraHandler("Внешний".into()),
                Usage::Reference
            ))
        );
    }

    #[test]
    fn extra_method_under_cursor_refers_to_api_handler_event() {
        let expected = SemanticInfo::new(
            Symbol::HandlerEvent {
                handler: "Модуль.Записи_АПИ".into(),
                event: "ДействиеА".into(),
            },
            Usage::Reference,
        );
        assert_eq!(at("Д$ействиеА"), Some(expected));
        assert_eq!(at("ДействиеБ $ ДействиеА"), None);
    }

    #[test]
    fn other_attributes_and_tags_are_ignored() {
        assert_eq!(at(r#"<Поле Имя="Ко$д"/>"#), None);
        assert_eq!(at(r#"<Other Имя="Про$чее"/>"#), None);
        assert_eq!(at("<Sel$ect"), None);
    }

    /// Expression at `$` inside `pattern`, which must occur once in `src`.
    fn expression_at(src: &str, pattern: &str) -> Option<RxExpression> {
        let pos = pattern.find('$').expect("pattern needs a `$` cursor");
        let needle = pattern.replace('$', "");
        assert_eq!(
            src.matches(&needle).count(),
            1,
            "`{needle}` must occur once"
        );
        let offset = src.find(&needle).unwrap() + pos;
        rx_expression_at(&parse(src).syntax(), TextSize::from(offset as u32))
    }

    const EXPRESSIONS: &str = r#"<Resources>
   <APIBrowser Имя="Б" Выражение="Вычислить(&apos;x&apos;, Другая())"/>
   <Поле Имя="П" Выражение="Третья()"/>
</Resources>"#;

    #[test]
    fn browser_expression_is_decoded_with_the_cursor_moved_along() {
        let expression = expression_at(EXPRESSIONS, "Др$угая").unwrap();
        assert_eq!(expression.code, "Вычислить('x', Другая())");
        let cursor = usize::from(expression.offset);
        assert!(
            expression.code[cursor..].starts_with("угая"),
            "{expression:?}"
        );
    }

    #[test]
    fn expressions_of_other_tags_and_other_attributes_are_ignored() {
        assert_eq!(expression_at(EXPRESSIONS, "Тре$тья"), None);
        assert_eq!(expression_at(EXPRESSIONS, r#"Имя="$Б""#), None);
    }

    #[test]
    fn decode_entities_moves_cursor_past_entities() {
        let text = "&quot;a&quot; &amp; b";
        // cursor on `b`
        let (decoded, cursor) = decode_entities(text, text.find('b').unwrap());
        assert_eq!(decoded, r#""a" & b"#);
        assert_eq!(&decoded[cursor..], "b");
        // cursor inside an entity lands right after the decoded char
        let (_, cursor) = decode_entities(text, 2);
        assert_eq!(cursor, 1);
    }
}
