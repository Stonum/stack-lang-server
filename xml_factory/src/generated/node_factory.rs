//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_rowan::AstNode;
use xml_syntax::{
    XmlSyntaxElement as SyntaxElement, XmlSyntaxNode as SyntaxNode, XmlSyntaxToken as SyntaxToken,
    *,
};
pub fn xml_attribute(name: XmlName) -> XmlAttributeBuilder {
    XmlAttributeBuilder {
        name,
        initializer: None,
    }
}
pub struct XmlAttributeBuilder {
    name: XmlName,
    initializer: Option<XmlAttributeInitializerClause>,
}
impl XmlAttributeBuilder {
    pub fn with_initializer(mut self, initializer: XmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> XmlAttribute {
        XmlAttribute::unwrap_cast(SyntaxNode::new_detached(
            XmlSyntaxKind::XML_ATTRIBUTE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn xml_attribute_initializer_clause(
    eq_token: SyntaxToken,
    value: XmlString,
) -> XmlAttributeInitializerClause {
    XmlAttributeInitializerClause::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_ATTRIBUTE_INITIALIZER_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn xml_cdata_section(
    cdata_start_token: SyntaxToken,
    cdata_end_token: SyntaxToken,
) -> XmlCdataSectionBuilder {
    XmlCdataSectionBuilder {
        cdata_start_token,
        cdata_end_token,
        content_token: None,
    }
}
pub struct XmlCdataSectionBuilder {
    cdata_start_token: SyntaxToken,
    cdata_end_token: SyntaxToken,
    content_token: Option<SyntaxToken>,
}
impl XmlCdataSectionBuilder {
    pub fn with_content_token(mut self, content_token: SyntaxToken) -> Self {
        self.content_token = Some(content_token);
        self
    }
    pub fn build(self) -> XmlCdataSection {
        XmlCdataSection::unwrap_cast(SyntaxNode::new_detached(
            XmlSyntaxKind::XML_CDATA_SECTION,
            [
                Some(SyntaxElement::Token(self.cdata_start_token)),
                self.content_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.cdata_end_token)),
            ],
        ))
    }
}
pub fn xml_closing_element(
    l_angle_token: SyntaxToken,
    slash_token: SyntaxToken,
    name: XmlName,
    r_angle_token: SyntaxToken,
) -> XmlClosingElement {
    XmlClosingElement::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_CLOSING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn xml_comment(
    comment_start_token: SyntaxToken,
    comment_end_token: SyntaxToken,
) -> XmlCommentBuilder {
    XmlCommentBuilder {
        comment_start_token,
        comment_end_token,
        content_token: None,
    }
}
pub struct XmlCommentBuilder {
    comment_start_token: SyntaxToken,
    comment_end_token: SyntaxToken,
    content_token: Option<SyntaxToken>,
}
impl XmlCommentBuilder {
    pub fn with_content_token(mut self, content_token: SyntaxToken) -> Self {
        self.content_token = Some(content_token);
        self
    }
    pub fn build(self) -> XmlComment {
        XmlComment::unwrap_cast(SyntaxNode::new_detached(
            XmlSyntaxKind::XML_COMMENT,
            [
                Some(SyntaxElement::Token(self.comment_start_token)),
                self.content_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.comment_end_token)),
            ],
        ))
    }
}
pub fn xml_element(
    opening: XmlOpeningElement,
    children: XmlElementList,
    closing: XmlClosingElement,
) -> XmlElement {
    XmlElement::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_ELEMENT,
        [
            Some(SyntaxElement::Node(opening.into_syntax())),
            Some(SyntaxElement::Node(children.into_syntax())),
            Some(SyntaxElement::Node(closing.into_syntax())),
        ],
    ))
}
pub fn xml_name(value_token: SyntaxToken) -> XmlName {
    XmlName::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn xml_opening_element(
    l_angle_token: SyntaxToken,
    name: XmlName,
    attributes: XmlAttributeList,
    r_angle_token: SyntaxToken,
) -> XmlOpeningElement {
    XmlOpeningElement::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_OPENING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn xml_processing_instruction(
    l_angle_question_token: SyntaxToken,
    target: XmlName,
    question_r_angle_token: SyntaxToken,
) -> XmlProcessingInstructionBuilder {
    XmlProcessingInstructionBuilder {
        l_angle_question_token,
        target,
        question_r_angle_token,
        content_token: None,
    }
}
pub struct XmlProcessingInstructionBuilder {
    l_angle_question_token: SyntaxToken,
    target: XmlName,
    question_r_angle_token: SyntaxToken,
    content_token: Option<SyntaxToken>,
}
impl XmlProcessingInstructionBuilder {
    pub fn with_content_token(mut self, content_token: SyntaxToken) -> Self {
        self.content_token = Some(content_token);
        self
    }
    pub fn build(self) -> XmlProcessingInstruction {
        XmlProcessingInstruction::unwrap_cast(SyntaxNode::new_detached(
            XmlSyntaxKind::XML_PROCESSING_INSTRUCTION,
            [
                Some(SyntaxElement::Token(self.l_angle_question_token)),
                Some(SyntaxElement::Node(self.target.into_syntax())),
                self.content_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.question_r_angle_token)),
            ],
        ))
    }
}
pub fn xml_prolog(
    xml_decl_start_token: SyntaxToken,
    attributes: XmlAttributeList,
    question_r_angle_token: SyntaxToken,
) -> XmlProlog {
    XmlProlog::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_PROLOG,
        [
            Some(SyntaxElement::Token(xml_decl_start_token)),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(question_r_angle_token)),
        ],
    ))
}
pub fn xml_root(content: XmlElementList, eof_token: SyntaxToken) -> XmlRootBuilder {
    XmlRootBuilder {
        content,
        eof_token,
        bom_token: None,
        prolog: None,
    }
}
pub struct XmlRootBuilder {
    content: XmlElementList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
    prolog: Option<XmlProlog>,
}
impl XmlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn with_prolog(mut self, prolog: XmlProlog) -> Self {
        self.prolog = Some(prolog);
        self
    }
    pub fn build(self) -> XmlRoot {
        XmlRoot::unwrap_cast(SyntaxNode::new_detached(
            XmlSyntaxKind::XML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                self.prolog
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.content.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn xml_self_closing_element(
    l_angle_token: SyntaxToken,
    name: XmlName,
    attributes: XmlAttributeList,
    slash_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> XmlSelfClosingElement {
    XmlSelfClosingElement::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_SELF_CLOSING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn xml_string(value_token: SyntaxToken) -> XmlString {
    XmlString::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn xml_text(value_token: SyntaxToken) -> XmlText {
    XmlText::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_TEXT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn xml_attribute_list<I>(items: I) -> XmlAttributeList
where
    I: IntoIterator<Item = AnyXmlAttribute>,
    I::IntoIter: ExactSizeIterator,
{
    XmlAttributeList::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_ATTRIBUTE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn xml_element_list<I>(items: I) -> XmlElementList
where
    I: IntoIterator<Item = AnyXmlElement>,
    I::IntoIter: ExactSizeIterator,
{
    XmlElementList::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn xml_bogus<I>(slots: I) -> XmlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    XmlBogus::unwrap_cast(SyntaxNode::new_detached(XmlSyntaxKind::XML_BOGUS, slots))
}
pub fn xml_bogus_attribute<I>(slots: I) -> XmlBogusAttribute
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    XmlBogusAttribute::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_BOGUS_ATTRIBUTE,
        slots,
    ))
}
pub fn xml_bogus_element<I>(slots: I) -> XmlBogusElement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    XmlBogusElement::unwrap_cast(SyntaxNode::new_detached(
        XmlSyntaxKind::XML_BOGUS_ELEMENT,
        slots,
    ))
}
