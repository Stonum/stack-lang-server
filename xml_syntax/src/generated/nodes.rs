//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    XmlLanguage as Language, XmlSyntaxElement as SyntaxElement,
    XmlSyntaxElementChildren as SyntaxElementChildren,
    XmlSyntaxKind::{self as SyntaxKind, *},
    XmlSyntaxList as SyntaxList, XmlSyntaxNode as SyntaxNode, XmlSyntaxToken as SyntaxToken,
    macros::map_syntax_node,
};
use biome_rowan::{
    AstNode, AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator, RawSyntaxKind, SyntaxKindSet, SyntaxResult, support,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl XmlAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlAttributeFields {
        XmlAttributeFields {
            name: self.name(),
            initializer: self.initializer(),
        }
    }
    pub fn name(&self) -> SyntaxResult<XmlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn initializer(&self) -> Option<XmlAttributeInitializerClause> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for XmlAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlAttributeFields {
    pub name: SyntaxResult<XmlName>,
    pub initializer: Option<XmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlAttributeInitializerClause {
    pub(crate) syntax: SyntaxNode,
}
impl XmlAttributeInitializerClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlAttributeInitializerClauseFields {
        XmlAttributeInitializerClauseFields {
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<XmlString> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for XmlAttributeInitializerClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlAttributeInitializerClauseFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<XmlString>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlCdataSection {
    pub(crate) syntax: SyntaxNode,
}
impl XmlCdataSection {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlCdataSectionFields {
        XmlCdataSectionFields {
            cdata_start_token: self.cdata_start_token(),
            content_token: self.content_token(),
            cdata_end_token: self.cdata_end_token(),
        }
    }
    pub fn cdata_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn cdata_end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for XmlCdataSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlCdataSectionFields {
    pub cdata_start_token: SyntaxResult<SyntaxToken>,
    pub content_token: Option<SyntaxToken>,
    pub cdata_end_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlClosingElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlClosingElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlClosingElementFields {
        XmlClosingElementFields {
            l_angle_token: self.l_angle_token(),
            slash_token: self.slash_token(),
            name: self.name(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<XmlName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for XmlClosingElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlClosingElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<XmlName>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlComment {
    pub(crate) syntax: SyntaxNode,
}
impl XmlComment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlCommentFields {
        XmlCommentFields {
            comment_start_token: self.comment_start_token(),
            content_token: self.content_token(),
            comment_end_token: self.comment_end_token(),
        }
    }
    pub fn comment_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn comment_end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for XmlComment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlCommentFields {
    pub comment_start_token: SyntaxResult<SyntaxToken>,
    pub content_token: Option<SyntaxToken>,
    pub comment_end_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlElementFields {
        XmlElementFields {
            opening: self.opening(),
            children: self.children(),
            closing: self.closing(),
        }
    }
    pub fn opening(&self) -> SyntaxResult<XmlOpeningElement> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn children(&self) -> XmlElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn closing(&self) -> SyntaxResult<XmlClosingElement> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for XmlElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlElementFields {
    pub opening: SyntaxResult<XmlOpeningElement>,
    pub children: XmlElementList,
    pub closing: SyntaxResult<XmlClosingElement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlName {
    pub(crate) syntax: SyntaxNode,
}
impl XmlName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlNameFields {
        XmlNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for XmlName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlOpeningElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlOpeningElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlOpeningElementFields {
        XmlOpeningElementFields {
            l_angle_token: self.l_angle_token(),
            name: self.name(),
            attributes: self.attributes(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<XmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> XmlAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for XmlOpeningElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlOpeningElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<XmlName>,
    pub attributes: XmlAttributeList,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlProcessingInstruction {
    pub(crate) syntax: SyntaxNode,
}
impl XmlProcessingInstruction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlProcessingInstructionFields {
        XmlProcessingInstructionFields {
            l_angle_question_token: self.l_angle_question_token(),
            target: self.target(),
            content_token: self.content_token(),
            question_r_angle_token: self.question_r_angle_token(),
        }
    }
    pub fn l_angle_question_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn target(&self) -> SyntaxResult<XmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn content_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn question_r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for XmlProcessingInstruction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlProcessingInstructionFields {
    pub l_angle_question_token: SyntaxResult<SyntaxToken>,
    pub target: SyntaxResult<XmlName>,
    pub content_token: Option<SyntaxToken>,
    pub question_r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlProlog {
    pub(crate) syntax: SyntaxNode,
}
impl XmlProlog {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlPrologFields {
        XmlPrologFields {
            xml_decl_start_token: self.xml_decl_start_token(),
            attributes: self.attributes(),
            question_r_angle_token: self.question_r_angle_token(),
        }
    }
    pub fn xml_decl_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn attributes(&self) -> XmlAttributeList {
        support::list(&self.syntax, 1usize)
    }
    pub fn question_r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for XmlProlog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlPrologFields {
    pub xml_decl_start_token: SyntaxResult<SyntaxToken>,
    pub attributes: XmlAttributeList,
    pub question_r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl XmlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlRootFields {
        XmlRootFields {
            bom_token: self.bom_token(),
            prolog: self.prolog(),
            content: self.content(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn prolog(&self) -> Option<XmlProlog> {
        support::node(&self.syntax, 1usize)
    }
    pub fn content(&self) -> XmlElementList {
        support::list(&self.syntax, 2usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for XmlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub prolog: Option<XmlProlog>,
    pub content: XmlElementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlSelfClosingElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlSelfClosingElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlSelfClosingElementFields {
        XmlSelfClosingElementFields {
            l_angle_token: self.l_angle_token(),
            name: self.name(),
            attributes: self.attributes(),
            slash_token: self.slash_token(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<XmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> XmlAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for XmlSelfClosingElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlSelfClosingElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<XmlName>,
    pub attributes: XmlAttributeList,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlString {
    pub(crate) syntax: SyntaxNode,
}
impl XmlString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlStringFields {
        XmlStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for XmlString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XmlText {
    pub(crate) syntax: SyntaxNode,
}
impl XmlText {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> XmlTextFields {
        XmlTextFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for XmlText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct XmlTextFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyXmlAttribute {
    XmlAttribute(XmlAttribute),
    XmlBogusAttribute(XmlBogusAttribute),
}
impl AnyXmlAttribute {
    pub fn as_xml_attribute(&self) -> Option<&XmlAttribute> {
        match &self {
            Self::XmlAttribute(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_bogus_attribute(&self) -> Option<&XmlBogusAttribute> {
        match &self {
            Self::XmlBogusAttribute(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyXmlElement {
    XmlBogusElement(XmlBogusElement),
    XmlCdataSection(XmlCdataSection),
    XmlComment(XmlComment),
    XmlElement(XmlElement),
    XmlProcessingInstruction(XmlProcessingInstruction),
    XmlSelfClosingElement(XmlSelfClosingElement),
    XmlText(XmlText),
}
impl AnyXmlElement {
    pub fn as_xml_bogus_element(&self) -> Option<&XmlBogusElement> {
        match &self {
            Self::XmlBogusElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_cdata_section(&self) -> Option<&XmlCdataSection> {
        match &self {
            Self::XmlCdataSection(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_comment(&self) -> Option<&XmlComment> {
        match &self {
            Self::XmlComment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_element(&self) -> Option<&XmlElement> {
        match &self {
            Self::XmlElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_processing_instruction(&self) -> Option<&XmlProcessingInstruction> {
        match &self {
            Self::XmlProcessingInstruction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_self_closing_element(&self) -> Option<&XmlSelfClosingElement> {
        match &self {
            Self::XmlSelfClosingElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_xml_text(&self) -> Option<&XmlText> {
        match &self {
            Self::XmlText(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for XmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlAttribute")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("XmlAttribute").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlAttribute> for SyntaxNode {
    fn from(n: XmlAttribute) -> Self {
        n.syntax
    }
}
impl From<XmlAttribute> for SyntaxElement {
    fn from(n: XmlAttribute) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlAttributeInitializerClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_ATTRIBUTE_INITIALIZER_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ATTRIBUTE_INITIALIZER_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlAttributeInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlAttributeInitializerClause")
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("XmlAttributeInitializerClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlAttributeInitializerClause> for SyntaxNode {
    fn from(n: XmlAttributeInitializerClause) -> Self {
        n.syntax
    }
}
impl From<XmlAttributeInitializerClause> for SyntaxElement {
    fn from(n: XmlAttributeInitializerClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlCdataSection {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_CDATA_SECTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_CDATA_SECTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlCdataSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlCdataSection")
                .field(
                    "cdata_start_token",
                    &support::DebugSyntaxResult(self.cdata_start_token()),
                )
                .field(
                    "content_token",
                    &support::DebugOptionalElement(self.content_token()),
                )
                .field(
                    "cdata_end_token",
                    &support::DebugSyntaxResult(self.cdata_end_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlCdataSection").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlCdataSection> for SyntaxNode {
    fn from(n: XmlCdataSection) -> Self {
        n.syntax
    }
}
impl From<XmlCdataSection> for SyntaxElement {
    fn from(n: XmlCdataSection) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlClosingElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_CLOSING_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_CLOSING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlClosingElement")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlClosingElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlClosingElement> for SyntaxNode {
    fn from(n: XmlClosingElement) -> Self {
        n.syntax
    }
}
impl From<XmlClosingElement> for SyntaxElement {
    fn from(n: XmlClosingElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlComment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_COMMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_COMMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlComment")
                .field(
                    "comment_start_token",
                    &support::DebugSyntaxResult(self.comment_start_token()),
                )
                .field(
                    "content_token",
                    &support::DebugOptionalElement(self.content_token()),
                )
                .field(
                    "comment_end_token",
                    &support::DebugSyntaxResult(self.comment_end_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlComment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlComment> for SyntaxNode {
    fn from(n: XmlComment) -> Self {
        n.syntax
    }
}
impl From<XmlComment> for SyntaxElement {
    fn from(n: XmlComment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlElement")
                .field("opening", &support::DebugSyntaxResult(self.opening()))
                .field("children", &self.children())
                .field("closing", &support::DebugSyntaxResult(self.closing()))
                .finish()
        } else {
            f.debug_struct("XmlElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlElement> for SyntaxNode {
    fn from(n: XmlElement) -> Self {
        n.syntax
    }
}
impl From<XmlElement> for SyntaxElement {
    fn from(n: XmlElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlName> for SyntaxNode {
    fn from(n: XmlName) -> Self {
        n.syntax
    }
}
impl From<XmlName> for SyntaxElement {
    fn from(n: XmlName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlOpeningElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_OPENING_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_OPENING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlOpeningElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlOpeningElement")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("attributes", &self.attributes())
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlOpeningElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlOpeningElement> for SyntaxNode {
    fn from(n: XmlOpeningElement) -> Self {
        n.syntax
    }
}
impl From<XmlOpeningElement> for SyntaxElement {
    fn from(n: XmlOpeningElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlProcessingInstruction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_PROCESSING_INSTRUCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_PROCESSING_INSTRUCTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlProcessingInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlProcessingInstruction")
                .field(
                    "l_angle_question_token",
                    &support::DebugSyntaxResult(self.l_angle_question_token()),
                )
                .field("target", &support::DebugSyntaxResult(self.target()))
                .field(
                    "content_token",
                    &support::DebugOptionalElement(self.content_token()),
                )
                .field(
                    "question_r_angle_token",
                    &support::DebugSyntaxResult(self.question_r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlProcessingInstruction").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlProcessingInstruction> for SyntaxNode {
    fn from(n: XmlProcessingInstruction) -> Self {
        n.syntax
    }
}
impl From<XmlProcessingInstruction> for SyntaxElement {
    fn from(n: XmlProcessingInstruction) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlProlog {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_PROLOG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_PROLOG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlProlog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlProlog")
                .field(
                    "xml_decl_start_token",
                    &support::DebugSyntaxResult(self.xml_decl_start_token()),
                )
                .field("attributes", &self.attributes())
                .field(
                    "question_r_angle_token",
                    &support::DebugSyntaxResult(self.question_r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlProlog").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlProlog> for SyntaxNode {
    fn from(n: XmlProlog) -> Self {
        n.syntax
    }
}
impl From<XmlProlog> for SyntaxElement {
    fn from(n: XmlProlog) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("prolog", &support::DebugOptionalElement(self.prolog()))
                .field("content", &self.content())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("XmlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlRoot> for SyntaxNode {
    fn from(n: XmlRoot) -> Self {
        n.syntax
    }
}
impl From<XmlRoot> for SyntaxElement {
    fn from(n: XmlRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlSelfClosingElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_SELF_CLOSING_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_SELF_CLOSING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlSelfClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlSelfClosingElement")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("attributes", &self.attributes())
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlSelfClosingElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlSelfClosingElement> for SyntaxNode {
    fn from(n: XmlSelfClosingElement) -> Self {
        n.syntax
    }
}
impl From<XmlSelfClosingElement> for SyntaxElement {
    fn from(n: XmlSelfClosingElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_STRING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlString> for SyntaxNode {
    fn from(n: XmlString) -> Self {
        n.syntax
    }
}
impl From<XmlString> for SyntaxElement {
    fn from(n: XmlString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for XmlText {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_TEXT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_TEXT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("XmlText")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("XmlText").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<XmlText> for SyntaxNode {
    fn from(n: XmlText) -> Self {
        n.syntax
    }
}
impl From<XmlText> for SyntaxElement {
    fn from(n: XmlText) -> Self {
        n.syntax.into()
    }
}
impl From<XmlAttribute> for AnyXmlAttribute {
    fn from(node: XmlAttribute) -> Self {
        Self::XmlAttribute(node)
    }
}
impl From<XmlBogusAttribute> for AnyXmlAttribute {
    fn from(node: XmlBogusAttribute) -> Self {
        Self::XmlBogusAttribute(node)
    }
}
impl AstNode for AnyXmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        XmlAttribute::KIND_SET.union(XmlBogusAttribute::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, XML_ATTRIBUTE | XML_BOGUS_ATTRIBUTE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            XML_ATTRIBUTE => Self::XmlAttribute(XmlAttribute { syntax }),
            XML_BOGUS_ATTRIBUTE => Self::XmlBogusAttribute(XmlBogusAttribute { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::XmlAttribute(it) => &it.syntax,
            Self::XmlBogusAttribute(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::XmlAttribute(it) => it.syntax,
            Self::XmlBogusAttribute(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyXmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XmlAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlBogusAttribute(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyXmlAttribute> for SyntaxNode {
    fn from(n: AnyXmlAttribute) -> Self {
        match n {
            AnyXmlAttribute::XmlAttribute(it) => it.into(),
            AnyXmlAttribute::XmlBogusAttribute(it) => it.into(),
        }
    }
}
impl From<AnyXmlAttribute> for SyntaxElement {
    fn from(n: AnyXmlAttribute) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<XmlBogusElement> for AnyXmlElement {
    fn from(node: XmlBogusElement) -> Self {
        Self::XmlBogusElement(node)
    }
}
impl From<XmlCdataSection> for AnyXmlElement {
    fn from(node: XmlCdataSection) -> Self {
        Self::XmlCdataSection(node)
    }
}
impl From<XmlComment> for AnyXmlElement {
    fn from(node: XmlComment) -> Self {
        Self::XmlComment(node)
    }
}
impl From<XmlElement> for AnyXmlElement {
    fn from(node: XmlElement) -> Self {
        Self::XmlElement(node)
    }
}
impl From<XmlProcessingInstruction> for AnyXmlElement {
    fn from(node: XmlProcessingInstruction) -> Self {
        Self::XmlProcessingInstruction(node)
    }
}
impl From<XmlSelfClosingElement> for AnyXmlElement {
    fn from(node: XmlSelfClosingElement) -> Self {
        Self::XmlSelfClosingElement(node)
    }
}
impl From<XmlText> for AnyXmlElement {
    fn from(node: XmlText) -> Self {
        Self::XmlText(node)
    }
}
impl AstNode for AnyXmlElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = XmlBogusElement::KIND_SET
        .union(XmlCdataSection::KIND_SET)
        .union(XmlComment::KIND_SET)
        .union(XmlElement::KIND_SET)
        .union(XmlProcessingInstruction::KIND_SET)
        .union(XmlSelfClosingElement::KIND_SET)
        .union(XmlText::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            XML_BOGUS_ELEMENT
                | XML_CDATA_SECTION
                | XML_COMMENT
                | XML_ELEMENT
                | XML_PROCESSING_INSTRUCTION
                | XML_SELF_CLOSING_ELEMENT
                | XML_TEXT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            XML_BOGUS_ELEMENT => Self::XmlBogusElement(XmlBogusElement { syntax }),
            XML_CDATA_SECTION => Self::XmlCdataSection(XmlCdataSection { syntax }),
            XML_COMMENT => Self::XmlComment(XmlComment { syntax }),
            XML_ELEMENT => Self::XmlElement(XmlElement { syntax }),
            XML_PROCESSING_INSTRUCTION => {
                Self::XmlProcessingInstruction(XmlProcessingInstruction { syntax })
            }
            XML_SELF_CLOSING_ELEMENT => {
                Self::XmlSelfClosingElement(XmlSelfClosingElement { syntax })
            }
            XML_TEXT => Self::XmlText(XmlText { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::XmlBogusElement(it) => &it.syntax,
            Self::XmlCdataSection(it) => &it.syntax,
            Self::XmlComment(it) => &it.syntax,
            Self::XmlElement(it) => &it.syntax,
            Self::XmlProcessingInstruction(it) => &it.syntax,
            Self::XmlSelfClosingElement(it) => &it.syntax,
            Self::XmlText(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::XmlBogusElement(it) => it.syntax,
            Self::XmlCdataSection(it) => it.syntax,
            Self::XmlComment(it) => it.syntax,
            Self::XmlElement(it) => it.syntax,
            Self::XmlProcessingInstruction(it) => it.syntax,
            Self::XmlSelfClosingElement(it) => it.syntax,
            Self::XmlText(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyXmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XmlBogusElement(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlCdataSection(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlComment(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlElement(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlProcessingInstruction(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlSelfClosingElement(it) => std::fmt::Debug::fmt(it, f),
            Self::XmlText(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyXmlElement> for SyntaxNode {
    fn from(n: AnyXmlElement) -> Self {
        match n {
            AnyXmlElement::XmlBogusElement(it) => it.into(),
            AnyXmlElement::XmlCdataSection(it) => it.into(),
            AnyXmlElement::XmlComment(it) => it.into(),
            AnyXmlElement::XmlElement(it) => it.into(),
            AnyXmlElement::XmlProcessingInstruction(it) => it.into(),
            AnyXmlElement::XmlSelfClosingElement(it) => it.into(),
            AnyXmlElement::XmlText(it) => it.into(),
        }
    }
}
impl From<AnyXmlElement> for SyntaxElement {
    fn from(n: AnyXmlElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyXmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyXmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlAttributeInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlCdataSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlOpeningElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlProcessingInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlProlog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlSelfClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct XmlBogus {
    syntax: SyntaxNode,
}
impl XmlBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for XmlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XmlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<XmlBogus> for SyntaxNode {
    fn from(n: XmlBogus) -> Self {
        n.syntax
    }
}
impl From<XmlBogus> for SyntaxElement {
    fn from(n: XmlBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct XmlBogusAttribute {
    syntax: SyntaxNode,
}
impl XmlBogusAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for XmlBogusAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_BOGUS_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_BOGUS_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlBogusAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XmlBogusAttribute")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<XmlBogusAttribute> for SyntaxNode {
    fn from(n: XmlBogusAttribute) -> Self {
        n.syntax
    }
}
impl From<XmlBogusAttribute> for SyntaxElement {
    fn from(n: XmlBogusAttribute) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct XmlBogusElement {
    syntax: SyntaxNode,
}
impl XmlBogusElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for XmlBogusElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_BOGUS_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_BOGUS_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for XmlBogusElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XmlBogusElement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<XmlBogusElement> for SyntaxNode {
    fn from(n: XmlBogusElement) -> Self {
        n.syntax
    }
}
impl From<XmlBogusElement> for SyntaxElement {
    fn from(n: XmlBogusElement) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyXmlBogusNode = XmlBogus | XmlBogusAttribute | XmlBogusElement }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct XmlAttributeList {
    syntax_list: SyntaxList,
}
impl XmlAttributeList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for XmlAttributeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_ATTRIBUTE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ATTRIBUTE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for XmlAttributeList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for XmlAttributeList {
    type Language = Language;
    type Node = AnyXmlAttribute;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for XmlAttributeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("XmlAttributeList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &XmlAttributeList {
    type Item = AnyXmlAttribute;
    type IntoIter = AstNodeListIterator<Language, AnyXmlAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for XmlAttributeList {
    type Item = AnyXmlAttribute;
    type IntoIter = AstNodeListIterator<Language, AnyXmlAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct XmlElementList {
    syntax_list: SyntaxList,
}
impl XmlElementList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for XmlElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(XML_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ELEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for XmlElementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for XmlElementList {
    type Language = Language;
    type Node = AnyXmlElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for XmlElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("XmlElementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &XmlElementList {
    type Item = AnyXmlElement;
    type IntoIter = AstNodeListIterator<Language, AnyXmlElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for XmlElementList {
    type Item = AnyXmlElement;
    type IntoIter = AstNodeListIterator<Language, AnyXmlElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
